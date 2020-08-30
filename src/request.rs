use serde::{Deserialize, Serialize, Serializer};
use serde_json::{json, Value};

use crate::client::Client;
use crate::error::{Error, Result};
use crate::response::{BlocklistUpdate, FreeSpace, PortTest, RpcResponse};

#[derive(Serialize, Deserialize)]
pub struct RpcRequest {
    pub method: Method,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<IDS>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all(serialize = "kebab-case"))]
pub enum Method {
    SessionGet,
    SessionStats,
    BlocklistUpdate,
    PortTest,
    SessionClose,
    FreeSpace,
    TorrentStart,
    TorrentStartNow,
    TorrentStop,
    TorrentVerify,
    TorrentReannounce,
    TorrentSet,
    TorrentGet,
    TorrentAdd,
    TorrentRemove,
    TorrentSetLocation,
    TorrentRenamePath,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ID {
    ID(i64),
    Hash(String),
}

#[derive(Debug, Deserialize)]
pub enum IDS {
    ID(i64),
    Array(Vec<ID>),
    RecentlyAdded,
}

impl Default for IDS {
    fn default() -> Self {
        IDS::RecentlyAdded
    }
}

impl Serialize for IDS {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            IDS::ID(id) => serializer.serialize_i64(*id),
            IDS::Array(values) => values.serialize(serializer),
            IDS::RecentlyAdded => serializer.serialize_str("recently-active"),
        }
    }
}

// #[derive(Serialize, Deserialize)]
// pub enum RequestArgs {
//     FreeSpaceArgs(FreeSpaceArgs),
// }

// #[derive(Serialize, Deserialize)]
// pub struct FreeSpaceArgs {
//     path: String,
// }

pub fn value_from_response(response: RpcResponse) -> Result<Value> {
    if &response.result == "success" {
        Ok(response
            .arguments
            .map_or(Err(Error::NoArguments), |v| Ok(v))?)
    } else {
        Err(Error::BadResponse(response.result))
    }
}

impl Client {
    pub async fn blocklist_update(&mut self) -> Result<BlocklistUpdate> {
        let request = RpcRequest {
            method: Method::BlocklistUpdate,
            arguments: None,
            tag: None,
        };
        let response = self.send_msg(&request).await?;
        let parsed_value = serde_json::from_value(value_from_response(response)?)?;
        Ok(parsed_value)
    }

    pub async fn port_test(&mut self) -> Result<PortTest> {
        let request = RpcRequest {
            method: Method::PortTest,
            arguments: None,
            tag: None,
        };
        let response = self.send_msg(&request).await?;
        let parsed_value = serde_json::from_value(value_from_response(response)?)?;
        Ok(parsed_value)
    }

    pub async fn free_space(&mut self, path: &str) -> Result<FreeSpace> {
        let request = RpcRequest {
            method: Method::FreeSpace,
            arguments: Some(json!({"path": path.to_string()})),
            tag: None,
        };
        let response = self.send_msg(&request).await?;
        let parsed_value = serde_json::from_value(value_from_response(response)?)?;
        Ok(parsed_value)
    }
}
