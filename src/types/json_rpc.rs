use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct JsonRpcRequest {
    #[serde(rename = "jsonrpc")]
    json_rpc: String,
    id: u64,
    method: String,
    params: Vec<Value>,
}

#[derive(Debug, Deserialize)]
pub struct JsonRpcResponse<T> {
    #[serde(rename = "jsonrpc")]
    pub json_rpc: String,
    pub id: u64,
    pub result: Option<T>,
    pub error: Option<Value>
}

impl JsonRpcRequest {
    pub fn new(method: String, params: Vec<Value>) -> Self {
        Self {
            json_rpc: "2.0".to_string(),
            id: 1,
            method: method,
            params: params,
        }
    }
}
