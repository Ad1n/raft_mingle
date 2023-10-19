use std::cell::OnceCell;
use std::fmt::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub static CORE_NODE: OnceCell<Arc<Mutex<Node>>> = OnceCell::new();

/// Raft consensus node
pub struct Node {
    pub id: u8,
    pub state: State,
    pub peer_ids: Vec<u8>,
    // client: Client,
}

impl Node {
    pub fn new(id: u8, state: State, peer_ids: Vec<u8>) -> Result<Node, Error> {
        Ok(Self {
            id,
            state,
            peer_ids,
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

        todo!()
    }
}