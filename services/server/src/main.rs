use std::error::Error;
use std::sync::{Arc, Mutex};
use consensus::raft::{Node, State};

fn main() -> Result<(), dyn Error> {
    consensus::raft::CORE_NODE.get_or_init(
        || Arc::new(
            Mutex::new(
                Node::new(
                    0,
                    State::Follower,
                    vec![]
                )?
            )
        )
    );

    println!("Hello, world!");

    Ok(())
}
