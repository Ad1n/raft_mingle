use std::sync::{Arc, Mutex};
use consensus::raft::Node;
use error::CustomError;

#[tokio::main]
async fn main() -> Result<(), CustomError> {
    consensus::raft::CORE_NODE.set(
        Arc::new(Mutex::new(Node::default()))
    ).expect("CORE_NODE Initialization failed");


    println!("Hello, raft!");

    Ok(())
}
