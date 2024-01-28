use consensus::raft;
use std::sync::Arc;
use std::sync::Mutex;
use storage::simple_storage::SimpleStorage;
use uuid::Uuid;

pub struct ServerCore {
    pub guard: Arc<Mutex<InnerData>>,
}

pub struct InnerData {
    id: Uuid,
    peer_ids: Vec<Uuid>,
    consensus: ConsensusAlgorithm,
    storage: SimpleStorage,
    rpc_clients: Vec<rpc::client::Client>,
}

pub enum ConsensusAlgorithm {
    Raft(raft::Node),
}

impl ServerCore {
    pub fn new(
        id: Uuid,
        peer_ids: Vec<Uuid>,
        consensus: ConsensusAlgorithm,
        clients_uri: Vec<hyper::Uri>,
    ) -> Self {
        Self {
            guard: Arc::new(Mutex::new(InnerData {
                id,
                peer_ids,
                consensus,
                storage: SimpleStorage::new(),
                rpc_clients: clients_uri
                    .into_iter()
                    .map(rpc::client::Client::new)
                    .collect(),
            })),
        }
    }
}
