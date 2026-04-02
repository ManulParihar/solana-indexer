use serde::de::DeserializeOwned;
use serde_json::{Value, json};

use crate::{
    rpc::{client::RpcClient, types::SignatureResponse},
    types::json_rpc::{JsonRpcRequest, JsonRpcResponse}
};

#[derive(Debug)]
pub enum RpcError {
    HttpError(reqwest::Error),
    Rpc(Value),
    EmptyResponse,
}

impl RpcClient {
    pub async fn get_slot(&self) -> Result<u64, RpcError> {
        let params = vec![json!({
            "commitment": "finalized"
        })];

        self.request::<u64>(
            "getSlot",
            params
        ).await
    }

    pub async fn get_signatures_for_address(&self, address: &String) -> Result<Vec<SignatureResponse>, RpcError> {
        let params = vec![
            json!(address),
            json!({
                "commitment": "finalized",
            })
        ];

        self
        .request::<Vec<SignatureResponse>>("getSignaturesForAddress", params)
        .await
    }

    pub async fn request<T>(&self, method: &str, params: Vec<Value>) -> Result<T, RpcError> 
    where T: DeserializeOwned {
        let body = JsonRpcRequest::new(method.to_string(), params);

        let response = self.client()
            .post(self.url())
            .json(&body)
            .send()
            .await?;

        // let value = response.text().await?;
        // println!("response value: {}", value);

        let value = response
            .json::<JsonRpcResponse<T>>()
            .await?;

        // Err(reqwest::get("http://invalid.url").await.unwrap_err())

        match (value.result, value.error) {
            (Some(val), _) => Ok(val),
            (None, Some(e)) => Err(RpcError::Rpc(e)),
            (None, None) => Err(RpcError::EmptyResponse),
        }
    }
}

impl From<reqwest::Error> for RpcError {
    fn from(value: reqwest::Error) -> Self {
        RpcError::HttpError(value)
    }
}

impl std::fmt::Display for RpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RpcError::Rpc(e) => write!(f, "RPC error: {}", e),
            RpcError::HttpError(e) => write!(f, "HTTP error: {}", e),
            RpcError::EmptyResponse => write!(f, "Empty response"),
        }
    }
}
