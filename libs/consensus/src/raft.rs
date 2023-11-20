use error::CustomError;
use std::{
    sync::{Arc, Mutex, MutexGuard, OnceLock},
    time::{Duration, Instant},
};
use crate::timer::Timer;

type Term = usize;

pub static CORE_NODE: OnceLock<Arc<Mutex<Node>>> = OnceLock::new();


/// Raft consensus node
#[derive(Debug)]
pub struct Node {
    pub id: u8,
    pub state: State,
    pub peer_ids: Vec<u8>,
    pub term: Term,
    // client: Client,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            id: 0,
            state: State::Follower,
            peer_ids: vec![],
            term: 0,
        }
    }
}

impl Node {
    pub fn new(
        id: u8,
        state: State,
        peer_ids: Vec<u8>,
        term: Term,
    ) -> Result<Node, CustomError> {
        Ok(Self {
            id,
            state,
            peer_ids,
            term,
        })
    }

    pub fn get_guarded<'a>() -> Result<MutexGuard<'a, Node>, CustomError> {
        match CORE_NODE.get() {
            Some(guard) => Ok(guard
                .lock()
                .map_err(|err| CustomError::new(&err.to_string()))?),
            None => Err(CustomError::new("CORE_NODE is not initialized")),
        }
    }

    pub fn execute_one_iteration(
        term: Term,
        election_reset_event: Instant,
    ) -> Result<bool, CustomError> {
        match Self::get_guarded() {
            Ok(node) => {
                if node.state == State::Leader {
                    return Ok(false);
                };
                if node.term != term {
                    return Ok(false);
                }
                if Instant::now().duration_since(election_reset_event) >= Timer::generate_election_duration_time() { return Ok(false) }

                Ok(true)
            },
            Err(err) => Err(err),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum State {
    Follower,
    Candidate,
    Leader,
}

pub struct Election<'a> {
    timer: Timer,
    node: &'a Node,
}
