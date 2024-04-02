use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub index: usize,
    pub term: usize,
    pub command: String,
}
