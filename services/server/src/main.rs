use consensus::raft::Node;
use error::SimpleResult;
use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use config::Config;
use consensus::server::{ServerCore, SERVER_CORE};
use log::{error, info};
use rpc::server::{serve_append_entries, serve_request_vote};
use storage::simple_storage::{SimpleStorage, STORAGE};
use tokio::sync::{Notify, RwLock};

#[tokio::main]
async fn main() -> SimpleResult<()> {
    pretty_env_logger::init();

    std::env::set_var("ID", "1"); //TODO: MOvE ME to debug
    std::env::set_var("PEER_IDS", "2,3");
    std::env::set_var("PORT", "8001");
    std::env::set_var("RPC_CLIENTS_PORTS", "8002,8003");

    let config = Config::from_env()?;

    STORAGE.get_or_init(|| Arc::new(RwLock::new(SimpleStorage::new())));

    SERVER_CORE.get_or_init(|| {
        Arc::new(RwLock::new(ServerCore::new(
            config.id,
            config.peer_ids(),
            config.clients_uris(),
        )))
    });

    consensus::raft::CORE_NODE
        .get_or_init(|| Arc::new(RwLock::new(Node::new(config.id, config.peer_ids()))));

    // Create a shared notification mechanism for graceful shutdown
    let shutdown_notify = Arc::new(Notify::new());
    let server_shutdown = shutdown_notify.clone();

    // Spawn Axum server in a separate async task
    let server_handle = tokio::spawn(async move {
        let app = Router::new()
            .route("/", get(|| async { "Hello, Raft!" }))
            .route("/request_vote", post(serve_request_vote))
            .route("/append_entries", post(serve_append_entries));

        let addr: SocketAddr = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            Config::try_from_env("PORT").expect("Port missing"),
        );
        info!("Listening on http://{}", &addr);
        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .expect("Failed to bind to address");

        axum::serve(listener, app).await.expect("Server run failed");
    });

    tokio::spawn(async move {
        if let Some(node) = consensus::raft::CORE_NODE.get() {
            let lock = node.write().await;
            Node::start_election_timeout(node.clone(), lock).await
        } else {
            error!("Unassigned node");
            panic!("Node is unassigned")
        }
    });

    // Listen for CTRL+C signal for graceful shutdown
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl+c");
    info!("CTRL+C received, shutting down...");
    shutdown_notify.notify_one(); // Trigger shutdown of the server

    // Await the server task to finish
    server_handle.await?;

    Ok(())
}
