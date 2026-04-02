use std::time::Duration;

use crate::{config::Config, indexer::poller::Poller, rpc::client::RpcClient};

pub struct Indexer {
    pub rpc_client: RpcClient,
    pub config: Config,
}

impl Indexer {
    pub fn new(rpc_client: RpcClient, config: Config) -> Self {
        Self { rpc_client, config }
    }

    pub async fn run(&self, interval: Duration) {
        let mut poller = Poller::new(self.rpc_client.clone(), interval);
        poller.poll_slots().await;
    }
}
