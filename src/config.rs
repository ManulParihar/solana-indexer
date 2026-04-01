pub struct Config {
    pub rpc_url: String,
    pub polling_interval: u64,
}

impl Config {
    pub fn new(rpc_url: String, polling_interval: u64) -> Config {
        Config { rpc_url, polling_interval }
    }
}
