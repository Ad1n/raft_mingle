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
}

pub enum ConsensusAlgorithm {
    Raft(raft::Node),
}
