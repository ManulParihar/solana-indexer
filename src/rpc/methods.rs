use reqwest::Error;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

use crate::{
    rpc::{client::RpcClient},
    types::json_rpc::{JsonRpcRequest, JsonRpcResponse}
};

impl RpcClient {
    pub async fn get_slot(&self) -> Result<u64, Error> {
        let params = vec![json!({
            "commitment": "finalized"
        })];

        self.request::<u64>(
            "get_slot",
            params
        ).await
    }

    pub async fn request<T>(&self, method: &str, params: Vec<Value>) -> Result<T, reqwest::Error> 
    where T: DeserializeOwned {
        let body = JsonRpcRequest::new(method.to_string(), params);

        let response = self.client()
            .post(self.url())
            .json(&body)
            .send()
            .await?;

        let value = response
            .json::<JsonRpcResponse<T>>()
            .await?;

        Ok(value.result.expect("RPC did not return any result"))
    }
}
