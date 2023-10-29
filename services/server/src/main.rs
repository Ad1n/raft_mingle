use std::error::Error;
use std::sync::{Arc, Mutex};
use consensus::raft::{Node, State};

#[tokio::main]
async fn main() -> Result<(), dyn Error> {
    consensus::raft::CORE_NODE.call_once(
        || Arc::new(
            Mutex::new(
                Node::new(
                    0,
                    State::Follower,
                    vec![],
                    0,
                )?
            )
        )
    );

    println!("Hello, raft!");

    Ok(())
}
