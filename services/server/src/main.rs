use consensus::raft::Node;
use error::SimpleResult;
use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use config::Config;
use consensus::rpc::server::{
    serve_append_entries, serve_install_snapshot, serve_request_vote,
};
use consensus::server::{ServerCore, SERVER_CORE};
use log::info;
use storage::simple_storage::{SimpleStorage, STORAGE};
use tokio::sync::{Notify, RwLock};

#[tokio::main]
async fn main() -> SimpleResult<()> {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    #[cfg(debug_assertions)]
    {
        std::env::set_var("ID", "1"); //TODO: MOvE ME to debug
        std::env::set_var("PEER_IDS", "2,3");
        std::env::set_var("PORT", "8001");
        std::env::set_var("RPC_CLIENTS_PORTS", "8002,8003");
    }

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

    let server_handle = tokio::spawn(async move {
        let app = Router::new()
            .route("/", get(|| async { "Hello, Raft!" }))
            .route("/request_vote", post(serve_request_vote))
            .route("/install_snapshot", post(serve_install_snapshot))
            .route("/append_entries", post(serve_append_entries));

        let addr: SocketAddr = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            Config::try_from_env("PORT").expect("Port missing"),
        );
        info!("Listening on http://{}", &addr);
        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .expect("Failed to bind to address");

        axum::serve(listener, app).await.expect("Server run failed");
    });

    tokio::spawn(async move { Node::start_election_timeout().await });

    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl+c");
    info!("CTRL+C received, shutting down...");
    server_shutdown.notify_one(); // Trigger shutdown of the server

    server_handle.await?;

    Ok(())
}
