use std::fmt;
use std::str::FromStr;

pub mod client;
pub mod server;

pub enum Endpoint {
    RequestVote,
    AppendEntries,
    InstallSnapshot,
}

impl FromStr for Endpoint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "request_vote" => Ok(Self::RequestVote),
            "append_entries" => Ok(Self::AppendEntries),
            "install_snapshot" => Ok(Self::InstallSnapshot),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variant_str = match self {
            Self::RequestVote => "request_vote",
            Self::AppendEntries => "append_entries",
            Self::InstallSnapshot => "install_snapshot",
        };
        write!(f, "{}", variant_str)
    }
}
