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
    let polling_interval = Duration::from_millis(400);
    
    let config = Config::new(rpc_url, polling_interval);
    let rpc_client = RpcClient::new(config.clone().rpc_url);
    
    let indexer = Indexer::new(rpc_client, config);
    indexer.run(polling_interval).await;
}
