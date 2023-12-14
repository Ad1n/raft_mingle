use std::fmt;
use std::str::FromStr;

pub mod client;
pub mod server;

enum Endpoint {
    RequestVote,
    RequestVoteResponse,
    AppendEntries,
    AppendEntriesResponse,
    InstallSnapshot,
    InstallSnapshotResponse,
}

impl FromStr for Endpoint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "request_vote" => Ok(Self::RequestVote),
            "request_vote_response" => Ok(Self::RequestVoteResponse),
            "append_entries" => Ok(Self::AppendEntries),
            "append_entries_response" => Ok(Self::AppendEntriesResponse),
            "install_snapshot" => Ok(Self::InstallSnapshot),
            "install_snapshot_response" => Ok(Self::InstallSnapshotResponse),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variant_str = match self {
            Self::RequestVote => "request_vote",
            Self::RequestVoteResponse => "request_vote_response",
            Self::AppendEntries => "append_entries",
            Self::AppendEntriesResponse => "append_entries_response",
            Self::InstallSnapshot => "install_snapshot",
            Self::InstallSnapshotResponse => "install_snapshot_response",
        };
        write!(f, "{}", variant_str)
    }
}
