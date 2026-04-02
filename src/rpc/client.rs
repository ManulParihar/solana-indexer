use reqwest::{Client};

#[derive(Clone)]
pub struct RpcClient {
    client: Client,
    rpc_url: String,
}

impl RpcClient {
    pub fn new(rpc_url: String) -> RpcClient {
        RpcClient {
            client:  Client::new(),
            rpc_url: rpc_url,
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn url(&self) -> &String {
        &self.rpc_url
    }
}
