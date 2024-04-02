use crate::client::{
    AppendEntriesRequest, AppendEntriesResponse, RequestVoteRequest, RequestVoteResponse,
};
use axum::Json;

pub async fn serve_request_vote(
    Json(payload): Json<RequestVoteRequest>,
) -> Json<RequestVoteResponse> {
    let server_current_term: usize = 5; // This should be fetched from the server's state
    let has_voted_for: Option<usize> = None; // This should also be fetched from the server's state
    let last_log_term: usize = 4; // Last log term in the server's log
    let last_log_index: usize = 10; // Last log index in the server's log

    let vote_granted = if payload.term < server_current_term {
        // The candidate's term is older than the server's term
        false
    } else if has_voted_for.is_some() && has_voted_for.unwrap() != payload.candidate_id {
        // The server has already voted for someone else in this term
        false
    } else if payload.last_log_term < last_log_term
        || (payload.last_log_term == last_log_term
            && payload.last_log_index < last_log_index)
    {
        // The candidate's log is not at least as up-to-date as the server's log
        false
    } else {
        // Grant vote
        true
    };

    // Here, you should also update the server's state, like setting the current term and votedFor

    Json(RequestVoteResponse {
        term: server_current_term, // You might want to update this based on the received term
        vote_granted,
    })
}

pub async fn serve_append_entries(
    Json(payload): Json<AppendEntriesRequest>,
) -> Json<AppendEntriesResponse> {
    // Implement your logic here based on `payload` FIXME term
    Json(AppendEntriesResponse {
        term: 1,
        success: true,
    })
}
