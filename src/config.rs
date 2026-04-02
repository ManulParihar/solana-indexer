use std::time::Duration;

#[derive(Clone)]
pub struct Config {
    pub rpc_url: String,
    pub polling_interval: Duration,
}

impl Config {
    pub fn new(rpc_url: String, polling_interval: Duration) -> Config {
        Config { rpc_url, polling_interval }
    }
}
