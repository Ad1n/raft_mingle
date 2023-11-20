use std::time::Duration;
use std::time::Instant;
use rand::Rng;
use error::CustomError;
use crate::raft::Node;

pub struct Timer {
    timeout_duration: Duration,
}

impl Timer {
    pub fn generate_election_duration_time() -> Duration {
        Duration::from_millis(150 + rand::thread_rng().gen_range(0..150))
    }

    async fn run(&self) -> Result<bool, CustomError> {
        let term = Node::get_guarded()?.term;
        let election_reset_event = Instant::now();
        let mut ticker = tokio::time::interval(Duration::from_millis(10));

        loop {
            ticker.tick().await;

            if Node::execute_one_iteration(term, election_reset_event)? {
                return Ok(false);
            }
        }
    }
}