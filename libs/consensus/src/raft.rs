pub mod command;

use crate::raft::command::Command;
use crate::rpc::client::{InstallSnapshotRequest, RequestVoteRequest};
use crate::server::SERVER_CORE;
use error::CustomError;
use log::{error, info};
use log_entry::LogEntry;
use std::{
    sync::{Arc, OnceLock},
    time::Instant,
};
use tokio::sync::RwLock;
use tokio::time::{self, Duration};

pub static CORE_NODE: OnceLock<Arc<RwLock<Node>>> = OnceLock::new();

pub fn try_get_core_node() -> Arc<RwLock<Node>> {
    match CORE_NODE.get() {
        None => panic!("Node is empty"),
        Some(inner) => inner.clone(),
    }
}

/// Raft consensus node
///
/// `Node` encapsulates the state and behavior of a Raft consensus algorithm participant.
/// It tracks the node's current role in the cluster (Leader, Follower, or Candidate), its term,
/// log entries, and other essential data for maintaining distributed consensus. This includes
/// information on known peers, the current leader (if any), and indices for log replication and
/// commitment. The `Node` is responsible for executing the Raft protocol's rules, participating
/// in leader election, log replication, and ensuring the cluster reaches consensus reliably and
/// efficiently.
#[derive(Debug)]
pub struct Node {
    pub id: usize,                  // Unique identifier for the node
    pub leader_id: Option<usize>, // Tracks the current leader's ID if known FIXME add logic for knowing leader
    pub state: State, // Track whether the node is a Leader, Follower, or Candidate
    pub peer_ids: Vec<usize>, // A list of the IDs of other nodes in the cluster; critical for communication
    pub current_term: usize, // Tracks the latest term the node knows; crucial for elections and consistency checks
    pub election_reset_at: Instant, // Likely tracks the last time the node has reset its election timeout; important for triggering new elections
    pub voted_for: Option<usize>, // Tracks the candidate ID this node voted for in the current term; aligns with Raft's requirements
    pub log: Vec<LogEntry>, // The log entries; this is central to Raft's log replication mechanism
    pub next_index: Vec<usize>, // Needed for the leader to track the next log entry to send to each follower; important for log replication
    pub match_index: Vec<usize>, // Needed for the leader to track the highest log entry known to be replicated on each follower; important for deciding when it's safe to commit entries
    pub commit_index: usize,     // Tracks the highest log entry known to be committed.
    pub last_applied: usize, // Tracks the highest log entry applied to the state machine.
}

impl Node {
    /// Creates a new `Node` instance with given id and peer IDs.
    /// Initially, all nodes are set to the Follower state, with their logs and terms initialized
    /// to the beginnings of their respective sequences. The election timeout and indices for log
    /// replication management are also initialized, preparing the node to participate in the
    /// Raft consensus process.
    pub fn new(id: usize, peer_ids: Vec<usize>) -> Self {
        let len = peer_ids.len();
        Self {
            id,
            leader_id: None,
            state: State::Follower, // Nodes start as Followers.
            peer_ids,
            current_term: 0,                   // Start in term 0.
            election_reset_at: Instant::now(), // Initialize election timeout.
            voted_for: None,                   // No vote cast initially.
            log: vec![],                       // Empty log at startup.
            next_index: vec![0; len],          // Placeholder; updated upon election.
            match_index: vec![0; len], // Placeholder; updated upon successful log replication.
            commit_index: 0,           // No entries committed initially.
            last_applied: 0,           // No entries applied to state machine initially.
        }
    }

    pub async fn start_election_timeout() {
        let node = try_get_core_node();

        loop {
            let timeout_duration =
                Duration::from_millis(150 + rand::random::<u64>() % 150);
            time::sleep(timeout_duration).await;

            let node_read_guard = node.read().await;

            // Check if the node is still a follower before attempting to become a candidate
            if node_read_guard.state == State::Follower {
                drop(node_read_guard);

                info!("Election timeout reached, transitioning to Candidate");
                match Node::become_candidate().await {
                    Ok(_) => {
                        info!("Successfully transitioned to Candidate state.");
                        break; // Successfully started candidate state, exit loop
                    },
                    Err(err) => {
                        error!("Error on becoming candidate: {}", err.to_string());
                        // Continue the loop to retry becoming a candidate
                    },
                }
            } else {
                // If not a follower, there's no need to continue the timeout loop
                info!("Node state changed, no longer a follower, stopping election timeout.");
                break;
            }
        }
    }

    pub async fn become_candidate() -> Result<(), CustomError> {
        let core_node = try_get_core_node();
        let mut node_write_guard = core_node.write().await;

        node_write_guard.state = State::Candidate;
        node_write_guard.current_term += 1;
        node_write_guard.voted_for = Some(node_write_guard.id);
        node_write_guard.election_reset_at = Instant::now(); // Resetting the election timer

        let self_vote = 1;
        let votes_needed = (node_write_guard.peer_ids.len() / 2) + 1; // Majority
        let term = node_write_guard.current_term;
        let last_log_index = node_write_guard.log.len().saturating_sub(1);
        let last_log_term = node_write_guard.log.last().map_or(0, |entry| entry.term);
        let candidate_id = node_write_guard.id;

        info!(
            "ID: {} becomes CANDIDATE at term: {}",
            node_write_guard.id, term
        );

        drop(node_write_guard);

        let server = SERVER_CORE
            .get()
            .ok_or(CustomError::new("Server is not initialized"))?
            .clone();
        let server = server.read().await;
        let rpc_clients = &server.rpc_clients;

        let futures: Vec<_> = rpc_clients
            .iter()
            .map(|client| {
                let request = RequestVoteRequest {
                    term,
                    candidate_id,
                    last_log_index,
                    last_log_term,
                };
                client.request_vote(request)
            })
            .collect();

        let results = futures_util::future::join_all(futures).await;
        drop(server);

        let votes: usize = results
            .into_iter()
            .filter_map(Result::ok)
            .filter(|request_vote_response| request_vote_response.vote_granted)
            .count()
            + self_vote;

        let lock = try_get_core_node();
        let mut node_write_guard = lock.write().await;

        if votes >= votes_needed && node_write_guard.state == State::Candidate {
            // Transition to leader
            node_write_guard.state = State::Leader;
            node_write_guard.next_index =
                vec![node_write_guard.log.len(); node_write_guard.peer_ids.len()];
            node_write_guard.match_index = vec![0; node_write_guard.peer_ids.len()];

            info!(
                "Node {} became leader in term {}",
                node_write_guard.id, node_write_guard.current_term
            );

            drop(node_write_guard);
        } else {
            // Remain or revert to follower state
            node_write_guard.state = State::Follower;
            info!(
                "Node {} failed to become leader in term {}, remains as Follower",
                node_write_guard.id, node_write_guard.current_term
            );

            drop(node_write_guard);

            //Restart the election timeout to handle possible future candidacy
            tokio::spawn(
                async move { Command::send(Command::StartElectionTimeout).await },
            );
        }

        Ok(())
    }

    // This method applies a snapshot to the node.
    pub async fn apply_snapshot(
        snapshot_request: InstallSnapshotRequest,
    ) -> Result<(), CustomError> {
        let node_arc = try_get_core_node();
        let mut node = node_arc.write().await;

        // Assuming you have a method to update the state machine to the snapshot's state
        // node.update_state_machine_to_snapshot(&snapshot_request.snapshot_data)?; FIXME update state machine

        node.current_term = snapshot_request.term;
        node.commit_index = snapshot_request.last_included_index;
        node.last_applied = snapshot_request.last_included_index;

        // Reset log to ensure it's consistent with the snapshot
        // This might mean keeping entries after last_included_index if such entries exist
        node.log = vec![]; // Simplified: real implementation may adjust based on snapshot details

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum State {
    Follower,
    Candidate,
    Leader,
}
