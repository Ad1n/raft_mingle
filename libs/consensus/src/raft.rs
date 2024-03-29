use crate::server::ServerCore;
use error::CustomError;
use log::{error, info};
use log_entry::LogEntry;
use rpc::client::RequestVoteRequest;
use std::sync::Weak;
use std::{
    sync::{Arc, Mutex, MutexGuard, OnceLock},
    time::Instant,
};
use tokio::sync::{RwLock, RwLockWriteGuard};
use tokio::time::{self, Duration};

pub static CORE_NODE: OnceLock<Arc<RwLock<Node>>> = OnceLock::new();

/// Raft consensus node
#[derive(Debug)]
pub struct Node {
    pub id: usize,
    pub state: State,
    pub peer_ids: Vec<usize>,
    pub current_term: usize,
    pub server: Weak<ServerCore>,
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
            server: Weak::new(),
            election_reset_at: Instant::now(),
            voted_for: None,
            log: vec![],
            next_index: vec![0; len], // Placeholder values, will be updated when becoming a leader
            match_index: vec![0; len], // Similarly, placeholder values
        }
    }

    pub async fn start_election_timeout(&mut self) {
        let timeout_duration = Duration::from_millis(150 + rand::random::<u64>() % 150);
        let mut interval = time::interval(timeout_duration);

        interval.tick().await; // Skip the first tick
        loop {
            interval.tick().await;
            if self.state == State::Follower {
                info!("Election timeout reached, transitioning to Candidate");
                match self.become_candidate().await {
                    Ok(r) => break,
                    Err(err) => {
                        error!("Error on becoming candidate: {}", err.to_string());
                    },
                };
            }
        }
    }

    pub async fn become_candidate(&mut self) -> Result<(), CustomError> {
        self.state = State::Candidate;
        self.current_term += 1;
        self.voted_for = Some(self.id);
        let self_vote = 1;
        let votes_needed = (self.peer_ids.len() / 2) + 1; // Majority
        let term = self.current_term;
        // self.election_reset_at = Instant::now();
        let last_log_index = self.log.len().saturating_sub(1);
        let last_log_term = self.log.last().map_or(0, |entry| entry.term);
        let candidate_id = self.id;

        info!("ID: {} becomes CANDIDATE at term: {}", self.id, term);

        let server = self
            .server
            .upgrade()
            .ok_or_else(|| CustomError::new("Server upgrade failed"))?;

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

        if votes >= votes_needed {
            self.state = State::Leader;
            info!(
                "Node {} became leader in term {}",
                self.id, self.current_term
            );
            // Correctly initialize next_index for each follower to the current length of the log_entry
            self.next_index = vec![self.log.len(); self.peer_ids.len()];

            // Initialize match_index for each follower to 0
            self.match_index = vec![0; self.peer_ids.len()];

            //TODO: Heartbeats
        } else {
            info!(
                "Node {} failed to become leader in term {}",
                self.id, self.current_term
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
