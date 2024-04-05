use crate::rpc::client::RpcClient;
use reqwest::Url;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;

pub static SERVER_CORE: OnceLock<Arc<RwLock<ServerCore>>> = OnceLock::new();

pub struct ServerCore {
    pub id: usize,
    pub peer_ids: Vec<usize>,
    pub rpc_clients: Vec<RpcClient>,
}

impl ServerCore {
    pub fn new(id: usize, peer_ids: Vec<usize>, clients_uris: Vec<Url>) -> Self {
        Self {
            id,
            peer_ids,
            rpc_clients: clients_uris.into_iter().map(RpcClient::new).collect(),
        }
    }

    pub fn peers_with_clients(&self) -> impl Iterator<Item = (&usize, &RpcClient)> {
        self.peer_ids
            .iter()
            .zip(self.rpc_clients.iter())
            .map(|(id, client)| (id, client))
    }
}
