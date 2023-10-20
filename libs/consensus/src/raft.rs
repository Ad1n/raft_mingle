use std::cell::OnceCell;
use std::fmt::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;

type Term = usize;

pub static CORE_NODE: OnceCell<Arc<Mutex<Node>>> = OnceCell::new();

/// Raft consensus node
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

    pub fn safely_get_term() -> Result<Term, Error> {
        Ok(match CORE_NODE.get() {
            Some(guard) => {
                match guard.lock() {
                    Ok(guarded_node) => {
                        guarded_node.term
                    },
                    Err(err) => todo!(),
                }
            },
            None => todo!(),
        })
    }
}

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
    fn run(&self) -> Result<bool,Error> {
        let term = Node::safely_get_term()?;

        Ok(true)
    }

}