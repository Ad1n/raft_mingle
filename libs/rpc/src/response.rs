use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum RpcResponse {
    RequestVote(RequestVoteResponse),
}

#[derive(Debug, Deserialize)]
pub struct RequestVoteResponse {}
