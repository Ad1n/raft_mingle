use consensus::raft::Node;
use error::CustomError;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), CustomError> {
    consensus::raft::CORE_NODE.get_or_init(|| Arc::new(Mutex::new(Node::default())));


    println!("Hello, raft!");

    Ok(())
}
