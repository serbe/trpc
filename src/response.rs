use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcResponse {
    pub result: String,
    pub arguments: Option<Value>,
    pub tag: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortTest {
    #[serde(rename = "port-is-open")]
    pub port_is_open: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlocklistUpdate {
    #[serde(rename = "blocklist-size")]
    pub blocklist_size: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FreeSpace {
    pub path: String,
    #[serde(rename = "size-bytes")]
    pub size_bytes: i64,
}

pub fn value_from_response(response: RpcResponse) -> Result<Value, Error> {
    if &response.result == "success" {
        Ok(response.arguments.map_or(Err(Error::NoArguments), Ok)?)
    } else {
        Err(Error::BadResponse(response.result))
    }
}
