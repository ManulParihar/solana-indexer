use dotenv::dotenv;
use std::{env, time::Duration};
use solana_indexer::{
    config::Config,
    indexer::runner::Indexer,
    rpc::client::RpcClient
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let rpc_url  = String::from(env::var("RPC_URL").expect("RPC_URL not set"));
    let sol_address = String::from(env::var("SOL_ADDRESS").expect("Address not set"));
    let polling_interval = Duration::from_secs(2);
    
    let config = Config::new(rpc_url, polling_interval, sol_address);
    let rpc_client = RpcClient::new(config.clone().rpc_url);
    
    let indexer = Indexer::new(rpc_client, config);
    indexer.run(polling_interval).await;
}
