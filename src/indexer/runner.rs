use std::time::Duration;
use tokio::time::sleep;

use crate::{config::Config, rpc::client::RpcClient};

pub struct Indexer {
    pub rpc_client: RpcClient,
    pub config: Config,
}

impl Indexer {
    pub fn new(rpc_client: RpcClient, config: Config) -> Self {
        Self { rpc_client, config }
    }

    pub async fn run(&self, interval: Duration) {
        loop {
            let slots = self.rpc_client.get_slot().await;
            match slots {
                Ok(s) => println!("Slot: {}", s),
                Err(e) => println!("Error polling slot: {}", e)
            };

            sleep(interval).await;
        }
    }
}
