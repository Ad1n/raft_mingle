use std::fmt::Error;
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};
use std::time::{Duration, Instant};

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

impl Node {
    pub fn new(
        id: u8,
        state: State,
        peer_ids: Vec<u8>,
        term: Term,
    ) -> Result<Node, Error> {
        Ok(Self {
            id,
            state,
            peer_ids,
            term,
        })
    }


    pub fn get_guarded<'a>() -> Result<MutexGuard<'a, Node>, Error> {
        Ok(match CORE_NODE.get() {
            Some(guard) => {
                match guard.lock() {
                    Ok(guarded_value) => {
                        guarded_value
                    },
                    Err(err) => todo!(),
                }
            },
            None => todo!(),
        })
    }

    pub fn execute_one_iteration(term: Term, election_reset_event: Instant) -> Result<bool, Error> {
        match Self::get_guarded() {
            Ok(node) => {
                if node.state == State::Leader { return Ok(false) };
                if node.term != term { return Ok(false) }
                // if Instant::now().duration_since(election_reset_event) >= timeout { return Ok(false) }
                Ok(true)
            },
            Err(err) => todo!(),
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
    async fn run(&self) -> Result<bool,Error> {
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