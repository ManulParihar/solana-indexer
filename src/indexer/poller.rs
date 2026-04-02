use std::time::Duration;
use tokio::time::sleep;

use crate::rpc::{client::RpcClient, types::SignatureResponse};

pub struct Poller {
    pub rpc_client: RpcClient,
    pub polling_interval: Duration,
    pub last_processed_slot: Option<u64>,
    pub address: String,
}

impl Poller {
    pub fn new(rpc_client: RpcClient, polling_interval: Duration, address: String) -> Self {
        Self { rpc_client, polling_interval, last_processed_slot: None, address }
    }

    pub async fn poll_slots(&mut self) {
        loop {
            match  self.rpc_client.get_slot().await {
                Ok(current_slot) => {
                    match self.last_processed_slot {
                        None => {
                            self.last_processed_slot = Some(current_slot);
                        }
                        Some(prev_slot) => {
                            if current_slot > prev_slot {
                                let signatures = match self.rpc_client
                                        .get_signatures_for_address(&self.address)
                                        .await
                                {
                                    Ok(s) => s,
                                    Err(e) => {
                                        println!("Error fetching signatures: {:?}", e);
                                        continue;
                                    }
                                };

                                for slot in (prev_slot + 1)..=current_slot {
                                    let filtered_signatures = signatures
                                        .iter()
                                        .filter(|s| s.slot == slot)
                                        .count();
                                    
                                    println!("Processing slot: {}", slot);
                                    println!("Found {} signatures for slot {}", filtered_signatures, slot);
                                }
                                self.last_processed_slot = Some(current_slot);
                                println!("Slot: {}", current_slot);
                            }
                        }
                    }
                },
                Err(e) => {
                    println!("Error polling slot: {}", e)
                }
            }

            sleep(self.polling_interval).await
        }
    }
}
