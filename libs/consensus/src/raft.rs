use crate::server::ServerCore;
use crate::timer::Timer;
use error::CustomError;
use std::cell::RefCell;
use std::sync::Weak;
use std::{
    sync::{Arc, Mutex, MutexGuard, OnceLock},
    time::Instant,
};

type Term = i64;

pub static CORE_NODE: OnceLock<Arc<Mutex<Node>>> = OnceLock::new();

/// Raft consensus node
#[derive(Debug)]
pub struct Node {
    pub id: u8,
    pub state: State,
    pub peer_ids: Vec<u8>,
    pub term: Term,
    pub server: RefCell<Weak<Mutex<ServerCore>>>,
    pub election_reset_at: Instant,
    pub voted_for: Option<u8>,
}

impl Node {
    pub fn new(id: u8, peer_ids: Vec<u8>) -> Self {
        Self {
            id,
            state: State::Follower,
            peer_ids,
            term: 0,
            server: RefCell::new(Weak::new()),
            election_reset_at: Instant::now(),
            voted_for: None,
        }
    }

    pub fn start_election(&mut self) -> Result<(), CustomError> {
        self.state = State::Candidate;
        self.term += 1;
        let current_term = *&self.term;
        self.election_reset_at = Instant::now();
        self.voted_for = Some(self.id);
        // TODO: Log becomming candidate

        let votes_received: u32 = 1;

        todo!()
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
                if Instant::now().duration_since(election_reset_event)
                    >= Timer::generate_election_duration_time()
                {
                    return Ok(false);
                }

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
