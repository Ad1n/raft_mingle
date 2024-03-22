use crate::raft;
use reqwest::Url;
use rpc::client::RpcClient;
use std::cell::RefCell;
use std::sync::{Mutex, Weak};
use storage::simple_storage::SimpleStorage;

pub struct ServerCore {
    pub id: usize,
    pub peer_ids: Vec<usize>,
    pub consensus: RefCell<Weak<Mutex<raft::Node>>>,
    pub storage: SimpleStorage,
    pub rpc_clients: Vec<RpcClient>,
}

impl ServerCore {
    pub fn new(id: usize, peer_ids: Vec<usize>, clients_uris: Vec<Url>) -> Self {
        Self {
            id,
            peer_ids,
            consensus: RefCell::new(Weak::new()),
            storage: SimpleStorage::new(),
            rpc_clients: clients_uris.into_iter().map(RpcClient::new).collect(),
        }
    }

    pub fn peers_with_clients(&self) -> impl Iterator<Item = (&usize, &RpcClient)> {
        self.peer_ids
            .iter()
            .zip(self.rpc_clients.iter())
            .map(|id, client| (id, client))
    }
}
