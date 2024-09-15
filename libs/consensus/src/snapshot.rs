use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub last_included_index: usize,
    pub last_included_term: usize,
    pub data: Vec<u8>, // This could be a serialized form of your state machine.
}
