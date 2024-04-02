use crate::server::SERVER_CORE;
use error::CustomError;
use log::{error, info};
use log_entry::LogEntry;
use rpc::client::RequestVoteRequest;
use std::{
    sync::{Arc, OnceLock},
    time::Instant,
};
use tokio::sync::{RwLock, RwLockWriteGuard};
use tokio::time::{self, Duration};

pub static CORE_NODE: OnceLock<Arc<RwLock<Node>>> = OnceLock::new();

pub fn try_get_core_node() -> Arc<RwLock<Node>> {
    match CORE_NODE.get() {
        None => panic!("Node is empty"),
        Some(inner) => inner.clone(),
    }
}

/// Raft consensus node
#[derive(Debug)]
pub struct Node {
    pub id: usize,
    pub state: State,
    pub peer_ids: Vec<usize>,
    pub current_term: usize,
    pub election_reset_at: Instant,
    pub voted_for: Option<usize>,
    pub log: Vec<LogEntry>,
    pub next_index: Vec<usize>,
    pub match_index: Vec<usize>,
}

impl Node {
    pub fn new(id: usize, peer_ids: Vec<usize>) -> Self {
        let len = peer_ids.len();
        Self {
            id,
            state: State::Follower,
            peer_ids,
            current_term: 0,
            election_reset_at: Instant::now(),
            voted_for: None,
            log: vec![],
            next_index: vec![0; len], // Placeholder values, will be updated when becoming a leader
            match_index: vec![0; len], // Similarly, placeholder values
        }
    }

    pub async fn start_election_timeout<'a>(
        node: Arc<RwLock<Node>>,
        lock: RwLockWriteGuard<'a, Node>,
    ) {
        drop(lock);

        let timeout_duration = Duration::from_millis(150 + rand::random::<u64>() % 150);
        let mut interval = time::interval(timeout_duration);

        interval.tick().await; // Skip the first tick
        loop {
            interval.tick().await;
            let node_write_guard = node.write().await;
            if node_write_guard.state == State::Follower {
                info!("Election timeout reached, transitioning to Candidate");
                match Node::become_candidate(node_write_guard).await {
                    Ok(_) => break,
                    Err(err) => {
                        error!("Error on becoming candidate: {}", err.to_string());
                    },
                };
            }
        }
    }

    pub async fn become_candidate<'a>(
        mut node_write_guard: RwLockWriteGuard<'a, Node>,
    ) -> Result<(), CustomError> {
        node_write_guard.state = State::Candidate;
        node_write_guard.current_term += 1;
        node_write_guard.voted_for = Some(node_write_guard.id);
        let self_vote = 1;
        let votes_needed = (node_write_guard.peer_ids.len() / 2) + 1; // Majority
        let term = node_write_guard.current_term;
        // self.election_reset_at = Instant::now();
        let last_log_index = node_write_guard.log.len().saturating_sub(1);
        let last_log_term = node_write_guard.log.last().map_or(0, |entry| entry.term);
        let candidate_id = node_write_guard.id;

        info!(
            "ID: {} becomes CANDIDATE at term: {}",
            node_write_guard.id, term
        );

        drop(node_write_guard);

        let server_arc = SERVER_CORE
            .get()
            .ok_or(CustomError::new("Server is not initialized"))
            .map(Arc::clone)?;
        let server = server_arc.read().await;

        let futures: Vec<_> = server
            .rpc_clients
            .iter()
            .map(|client| {
                let term = term; //TODO: Shadow to move into async block
                let candidate_id = candidate_id;
                let last_log_index = last_log_index;
                let last_log_term = last_log_term;
                async move {
                    client
                        .request_vote(RequestVoteRequest {
                            term,
                            candidate_id,
                            last_log_index,
                            last_log_term,
                        })
                        .await
                }
            })
            .collect();

        let results = futures_util::future::join_all(futures).await;

        let votes: usize = results
            .into_iter()
            .filter_map(Result::ok)
            .filter(|request_vote_response| request_vote_response.vote_granted)
            .count()
            + 1; // Include self-vote

        let lock = try_get_core_node();
        let mut node_write_guard = lock.write().await;

        if votes >= votes_needed {
            node_write_guard.state = State::Leader;
            info!(
                "Node {} became leader in term {}",
                node_write_guard.id, node_write_guard.current_term
            );
            // Correctly initialize next_index for each follower to the current length of the log_entry
            node_write_guard.next_index =
                vec![node_write_guard.log.len(); node_write_guard.peer_ids.len()];

            // Initialize match_index for each follower to 0
            node_write_guard.match_index = vec![0; node_write_guard.peer_ids.len()];

            //TODO: Heartbeats
        } else {
            info!(
                "Node {} failed to become leader in term {}",
                node_write_guard.id, node_write_guard.current_term
            );
            // Handle election failure (e.g., revert to follower state)
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum State {
    Follower,
    Candidate,
    Leader,
}
