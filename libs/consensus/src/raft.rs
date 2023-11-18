use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};
use tokio::sync;
use error::CustomError;

type Term = usize;

pub static CORE_NODE: sync::OnceCell<Arc<Mutex<Node>>> = sync::OnceCell::const_new();


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
            Some(guard) => Ok(guard.lock().map_err(|err| CustomError::new(&err.to_string()))?),
            None => Err(CustomError::new("CORE_NODE is not initialized")),
        }
    }

    pub fn execute_one_iteration(term: Term, election_reset_event: Instant) -> Result<bool, CustomError> {
        match Self::get_guarded() {
            Ok(node) => {
                if node.state == State::Leader { return Ok(false) };
                if node.term != term { return Ok(false) }
                // if Instant::now().duration_since(election_reset_event) >= timeout { return Ok(false) }
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

struct Timer {
    timeout_duration: Duration,
}

impl Timer {
    async fn run(&self) -> Result<bool,CustomError> {
        let term = Node::get_guarded()?.term;
        let election_reset_event = Instant::now();
        let mut ticker = tokio::time::interval(Duration::from_millis(10));

        loop {
            ticker.tick().await;

            if Node::execute_one_iteration(term, election_reset_event)? { return Ok(false) }
        }

        Ok(true)
    }

}