use consensus::raft;
use std::sync::Arc;
use std::sync::Mutex;
use uuid::Uuid;

pub struct ServerCore {
    pub guard: Arc<Mutex<InnerData>>,
}

pub struct InnerData {
    id: Uuid,
    peer_ids: Vec<Uuid>,
    consensus: ConsensusAlgorithm,
    // rpc_server:
    // rpc_clients:
}

pub enum ConsensusAlgorithm {
    Raft(raft::Node),
}

impl ServerCore {
    pub fn new(id: Uuid, peer_ids: Vec<Uuid>, consensus: ConsensusAlgorithm) -> Self {
        Self {
            guard: Arc::new(Mutex::new(InnerData {
                id,
                peer_ids,
                consensus,
            })),
        }
    }
}
