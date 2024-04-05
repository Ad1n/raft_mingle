use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogEntry {
    pub index: usize,
    pub term: usize,
    pub command: String,
}
