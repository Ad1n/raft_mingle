use crate::rpc::Endpoint;
use log_entry::LogEntry;
use reqwest::{Client as ReqwestClient, Error as ReqwestError, Url};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::io;
use thiserror::Error;

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

    pub async fn send_append_entries(
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
    InstallSnapshot(InstallSnapshotRequest),
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

#[derive(Serialize, Deserialize)]
pub struct InstallSnapshotRequest {
    pub term: usize,                // leader's term
    pub leader_id: usize,           // so follower can redirect clients
    pub last_included_index: usize, // the snapshot replaces all entries up through and including this index
    pub last_included_term: usize,  // term of last_included_index
    pub data: Vec<u8>,              // snapshot data
}

#[derive(Deserialize)]
pub enum RpcResponse {
    RequestVote(RequestVoteResponse),
    AppendEntries(AppendEntriesResponse),
    InstallSnapshot(InstallSnapshotResponse),
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

#[derive(Serialize, Deserialize)]
pub struct InstallSnapshotResponse {
    pub term: usize, // currentTerm, for leader to update itself
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
