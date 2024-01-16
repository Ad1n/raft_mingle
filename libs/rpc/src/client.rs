use crate::response::RpcResponse;
use crate::Endpoint;
use bytes::Buf;
use getset::Getters;
use http_body_util::BodyExt;
use hyper;
use hyper::{Request, StatusCode};
use hyper_util::rt::TokioIo;
use std::io;
use thiserror::Error;
use tokio::net::TcpStream;

#[derive(Debug, Clone, Getters)]
pub struct Client {
    #[get = "pub"]
    uri: hyper::Uri,
}

impl Client {
    pub fn new(&self, uri: hyper::Uri) -> Self {
        Self { uri }
    }

    pub async fn send(
        &self,
        path: Endpoint,
        body: String,
    ) -> Result<RpcResponse, RpcClientError> {
        let url_with_path = format!("{}/{}", self.uri().to_string(), path);
        let uri = url_with_path.parse::<hyper::Uri>().unwrap();
        let stream = TcpStream::connect(uri.to_string()).await?;
        let io = TokioIo::new(stream);

        let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
        tokio::task::spawn(async move {
            if let Err(err) = conn.await {
                println!("Connection failed: {:?}", err);
            }
        });

        let authority = uri.authority().unwrap().clone();

        let req = Request::builder()
            .uri(uri)
            .header(hyper::header::HOST, authority.as_str())
            .body(body)?;

        let res = sender.send_request(req).await?;

        let body = res.collect().await?.aggregate();

        // try to parse as json with serde_json
        let response: RpcResponse = serde_json::from_reader(Buf::reader(body))?;

        Ok(response)
    }
}

#[derive(Error, Debug)]
pub enum RpcClientError {
    #[error("Parse config error")]
    IO(#[from] io::Error),
    #[error("Hyper Error")]
    InnerClient(#[from] hyper::Error),
    #[error("Deserialization error")]
    Deserialization(#[from] serde_json::Error),
    #[error("Request builder failed")]
    RequestBuilder(#[from] hyper::http::Error),
    #[error("Response failed {status:?} {body:?}")]
    ResponseFailed { body: String, status: StatusCode },
}
