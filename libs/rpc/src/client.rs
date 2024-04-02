use crate::Endpoint;
use axum::body::Body;
use axum::extract::path::ErrorKind;
use axum::extract::path::ErrorKind::ParseError;
use axum::http::{Request, Uri};
use bytes::Buf;
use error::CustomError;
use getset::Getters;
use http_body_util::BodyExt;
use hyper_util::rt::TokioIo;
use log_entry::LogEntry;
use reqwest::{Client as ReqwestClient, Client, Error as ReqwestError, Url};
use serde::de::{DeserializeOwned, Error};
use serde::{Deserialize, Serialize};
use std::backtrace::Backtrace;
use std::io;
use std::str::FromStr;
use thiserror::Error;
use tokio::net::TcpStream;

#[derive(Debug, Clone)]
pub struct RpcClient {
    base_uri: Url,
    client: ReqwestClient,
}

impl RpcClient {
    pub fn new(base_uri: Url) -> Self {
        Self {
            base_uri,
            client: ReqwestClient::new(),
        }
    }

    async fn send<Req, Res>(
        &self,
        endpoint: Endpoint,
        request: Req,
    ) -> Result<Res, RpcClientError>
    where
        Req: Serialize + Send + 'static,
        Res: DeserializeOwned + Send,
    {
        let uri = match self.base_uri.join(&endpoint.to_string()) {
            Ok(uri) => uri,
            Err(err) => {
                return Err(RpcClientError::UrlParseError {
                    message: err.to_string(),
                })
            },
        };
        let response = self
            .client
            .post(uri)
            .json(&request)
            .send()
            .await?
            .json::<Res>()
            .await?;
        Ok(response)
    }

    pub async fn request_vote(
        &self,
        request: RequestVoteRequest,
    ) -> Result<RequestVoteResponse, RpcClientError> {
        let response = self.send(Endpoint::RequestVote, request).await?;
        Ok(response)
    }

    pub async fn send_heartbeat(
        &self,
        request: AppendEntriesRequest,
    ) -> Result<AppendEntriesResponse, RpcClientError> {
        let response = self.send(Endpoint::AppendEntries, request).await?;
        Ok(response)
    }
}

#[derive(Serialize, Deserialize)]
pub enum RpcRequest {
    RequestVote(RequestVoteRequest),
    AppendEntries(AppendEntriesRequest),
}

#[derive(Serialize, Deserialize)]
pub struct RequestVoteRequest {
    pub term: usize,
    pub candidate_id: usize,
    pub last_log_index: usize,
    pub last_log_term: usize,
}

#[derive(Serialize, Deserialize)]
pub struct AppendEntriesRequest {
    pub term: usize,
    pub leader_id: usize,
    pub prev_log_index: usize,
    pub prev_log_term: usize,
    pub entries: Vec<LogEntry>,
    pub leader_commit: usize,
}

#[derive(Deserialize)]
pub enum RpcResponse {
    RequestVote(RequestVoteResponse),
    AppendEntries(AppendEntriesResponse),
}

#[derive(Serialize, Deserialize)]
pub struct RequestVoteResponse {
    pub term: usize,
    pub vote_granted: bool,
}

#[derive(Serialize, Deserialize)]
pub struct AppendEntriesResponse {
    pub term: usize,
    pub success: bool,
}

#[derive(Error, Debug)]
pub enum RpcClientError {
    #[error("Parse config error")]
    IO(#[from] io::Error),
    // #[error("Hyper Error")]
    // InnerClient(#[from] hyper::Error),
    #[error("Deserialization error")]
    Deserialization(#[from] serde_json::Error),
    // #[error("Request builder failed")]
    // RequestBuilder(#[from] hyper::http::Error),
    // #[error("Response failed {status:?} {body:?}")]
    // ResponseFailed { body: String, status: StatusCode },
    #[error("Request error")]
    RequestError(#[from] ReqwestError),
    #[error("URL parse error: {message:?}")]
    UrlParseError { message: String },
}
