use consensus::raft::Node;
use error::SimpleResult;
use std::sync::{Arc, Mutex};

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use config::Config;
use hyper::{server::conn::http1, service::service_fn};
use hyper_util::rt::TokioIo;
use log::{error, info};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> SimpleResult<()> {
    pretty_env_logger::init();

    std::env::set_var("ID", "1");
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

    let addr: SocketAddr = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        Config::try_from_env("PORT")?,
    );
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on http://{}", addr);
    info!("Hello, raft!");

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            let service = service_fn(move |req| rpc::server::serve_endpoints(req));

            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                error!("Failed to serve connection: {:?}", err);
            }
        });
    }
}
