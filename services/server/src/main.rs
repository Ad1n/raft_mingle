pub mod core;

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

    //-- config services/server/config.yml
    let config = config::initialize_config::<Config>(
        "Raft consensus sandbox",
        "Just a raft consensus implementation providing example of how it works",
    );
    consensus::raft::CORE_NODE.get_or_init(|| Arc::new(Mutex::new(Node::default())));

    let addr: SocketAddr = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        config.try_port_from_env()?,
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
