use serde::{Deserialize, Serialize};
// use serde_json::Value;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignatureResponse {
    pub signature: String,
    pub slot: u64,
    // #[serde(rename = "err")]
    // error: Option<Value>,
    // memo: Option<String>,
    // block_time: Option<i64>,
    // confirmation_status: Option<String>,
}
