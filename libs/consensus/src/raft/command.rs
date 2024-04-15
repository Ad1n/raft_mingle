use crate::raft::Node;
use log::error;
use once_cell::sync::Lazy;
use std::fmt::Display;
use std::sync::Arc;
use tokio::sync::mpsc;

static COMMAND_SENDER: Lazy<Arc<mpsc::Sender<Command>>> = Lazy::new(|| {
    let (sender, receiver) = mpsc::channel(100);
    tokio::spawn(async move {
        Command::process(receiver).await;
    });
    Arc::new(sender)
});

#[derive(Debug, Clone, Copy)]
pub enum Command {
    StartElectionTimeout,
}

impl Command {
    pub async fn process(mut commands: mpsc::Receiver<Command>) {
        while let Some(command) = commands.recv().await {
            match command {
                Self::StartElectionTimeout => Node::start_election_timeout().await,
            }
        }
    }

    pub async fn send(command: Command) {
        if Arc::clone(&*COMMAND_SENDER).send(command).await.is_err() {
            error!("Failed to send command: {:?}", command);
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::StartElectionTimeout => write!(f, "StartElectionTimeout"),
        }
    }
}
