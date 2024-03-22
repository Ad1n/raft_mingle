use consensus::raft::Node;
use error::SimpleResult;
use std::sync::{Arc, Mutex};

use axum::routing::{get, post};
use axum::Router;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use config::Config;
use hyper_util::rt::TokioIo;
use log::{error, info};
use rpc::server::{serve_append_entries, serve_request_vote};
use tokio::net::TcpListener;
use tokio::sync::Notify;

#[tokio::main]
async fn main() -> SimpleResult<()> {
    pretty_env_logger::init();

    std::env::set_var("ID", "1"); //TODO: MOvE ME to debug
    std::env::set_var("PEER_IDS", "2,3");
    std::env::set_var("PORT", "8001");
    std::env::set_var("RPC_CLIENTS_PORTS", "8002,8003");

    let config = Config::from_env()?;

    consensus::raft::CORE_NODE.get_or_init(|| {
        let consensus = Arc::new(Mutex::new(Node::new(config.id, config.peer_ids())));

        let server = Arc::new(Mutex::new(consensus::server::ServerCore::new(
            config.id,
            config.peer_ids(),
            config.clients_uris(),
        )));

        *server.lock().unwrap().consensus.borrow_mut() = Arc::downgrade(&consensus);
        *consensus.lock().unwrap().server.borrow_mut() = Arc::downgrade(&server);
        consensus
    });

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
            Config::try_from_env("PORT")?,
        );
        info!("Listening on http://{}", &addr);
        let listener = tokio::net::TcpListener::bind(addr).await?;

        axum::serve(listener, app).await?;
    });

    tokio::spawn(async move {
        consensus::raft::CORE_NODE
            .get()
            .map(|node| match node.clone().lock() {
                Ok(mut node) => node.start_election_timeout(),
                Err(err) => return err,
            })
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
