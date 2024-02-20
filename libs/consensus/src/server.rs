use crate::raft;
use std::cell::RefCell;
use std::sync::{Mutex, Weak};
use storage::simple_storage::SimpleStorage;

pub struct ServerCore {
    pub id: u8,
    pub peer_ids: Vec<u8>,
    pub consensus: RefCell<Weak<Mutex<raft::Node>>>,
    pub storage: SimpleStorage,
    pub rpc_clients: Vec<rpc::client::Client>,
}

impl ServerCore {
    pub fn new(id: u8, peer_ids: Vec<u8>, clients_uris: Vec<hyper::Uri>) -> Self {
        Self {
            id,
            peer_ids,
            consensus: RefCell::new(Weak::new()),
            storage: SimpleStorage::new(),
            rpc_clients: clients_uris
                .into_iter()
                .map(rpc::client::Client::new)
                .collect(),
        }
    }
}
