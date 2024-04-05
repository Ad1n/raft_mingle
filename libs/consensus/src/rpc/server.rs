use crate::rpc::client::{
    AppendEntriesRequest, AppendEntriesResponse, RequestVoteRequest, RequestVoteResponse,
};
use axum::Json;

pub async fn serve_request_vote(
    Json(payload): Json<RequestVoteRequest>,
) -> Json<RequestVoteResponse> {
    let node = crate::raft::try_get_core_node();
    let mut node = node.write().await;

    let current_term = node.current_term;

    // Assuming you have a method to get the last log term and index
    let last_log_index = node.log.last().map(|entry| entry.index).unwrap_or(0); //FIXME fn for this
    let last_log_term = node.log.last().map(|entry| entry.term).unwrap_or(0);

    let mut vote_granted = false;

    if payload.term < current_term {
        // Candidate's term is older, don't grant vote
    } else {
        if payload.term > current_term {
            // Candidate's term is newer, update current term and reset voted_for
            node.current_term = payload.term;
            node.voted_for = None;
        }

        if node.voted_for.is_none() || node.voted_for == Some(payload.candidate_id) {
            if payload.last_log_term > last_log_term
                || (payload.last_log_term == last_log_term
                    && payload.last_log_index >= last_log_index)
            {
                // Candidate's log is at least as up-to-date as receiver's log, grant vote
                vote_granted = true;
                node.voted_for = Some(payload.candidate_id);
            }
        }
    }

    Json(RequestVoteResponse {
        term: node.current_term, // Updated term
        vote_granted,
    })
}

pub async fn serve_append_entries(
    Json(payload): Json<AppendEntriesRequest>,
) -> Json<AppendEntriesResponse> {
    let node = crate::raft::try_get_core_node();
    let mut node = node.write().await;

    if payload.term < node.current_term {
        // If the term is outdated, reject the request
        return Json(AppendEntriesResponse {
            term: node.current_term,
            success: false,
        });
    }

    if let Some(last_log) = node.log.last() {
        if payload.prev_log_index > last_log.index
            || payload.prev_log_term != last_log.term
        {
            // If the log doesn't contain an entry at prev_log_index whose term matches prev_log_term, reject
            return Json(AppendEntriesResponse {
                term: node.current_term,
                success: false,
            });
        }
    }

    // If an existing entry conflicts with a new one (same index but different terms),
    // delete the existing entry and all that follow it
    let conflict_index = node.log.iter().position(|entry| {
        entry.index == payload.prev_log_index + 1 && entry.term != payload.prev_log_term
    });
    if let Some(index) = conflict_index {
        node.log.truncate(index);
    }

    // Append any new entries not already in the log
    if !payload.entries.is_empty() {
        node.log.extend_from_slice(&payload.entries);
    }

    // If leaderCommit > commitIndex, set commitIndex = min(leaderCommit, index of last new entry)
    if payload.leader_commit > node.commit_index {
        node.commit_index = std::cmp::min(
            payload.leader_commit,
            node.log.last().map_or(0, |entry| entry.index), //FIXME: todo fn for last entry
        );
        // Here you might want to apply log entries to the state machine
    }

    Json(AppendEntriesResponse {
        term: node.current_term,
        success: true,
    })
}
