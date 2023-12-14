use crate::Endpoint;

pub struct Client {
    uri: hyper::Uri,
}

impl Client {
    pub fn new(&self, endpoint: Endpoint, protocol: Protocol, port: u16, domain: Domain) -> Self {
        let protocol = Protocol
    }
}
