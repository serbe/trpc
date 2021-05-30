use serde::{Deserialize, Serialize, Serializer};
use serde_json::{json, Value};

use crate::client::Client;
use crate::error::{Error};
use crate::response::{BlocklistUpdate, FreeSpace, PortTest, RpcResponse};

#[derive(Serialize, Deserialize)]
pub struct RpcRequest {
    pub method: Method,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Ids>,
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
pub enum Id {
    Id(i64),
    Hash(String),
}

#[derive(Debug, Deserialize)]
pub enum Ids {
    Id(i64),
    Array(Vec<Id>),
    RecentlyAdded,
}

impl Default for Ids {
    fn default() -> Self {
        Ids::RecentlyAdded
    }
}

impl Serialize for Ids {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Ids::Id(id) => serializer.serialize_i64(*id),
            Ids::Array(values) => values.serialize(serializer),
            Ids::RecentlyAdded => serializer.serialize_str("recently-active"),
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

pub fn value_from_response(response: RpcResponse) -> Result<Value, Error> {
    if &response.result == "success" {
        Ok(response
            .arguments
            .map_or(Err(Error::NoArguments), Ok)?)
    } else {
        Err(Error::BadResponse(response.result))
    }
}

impl Client {
    pub async fn blocklist_update(&mut self) -> Result<BlocklistUpdate, Error> {
        let request = RpcRequest {
            method: Method::BlocklistUpdate,
            arguments: None,
            tag: None,
        };
        let response = self.send_msg(&request).await?;
        let parsed_value = serde_json::from_value(value_from_response(response)?)?;
        Ok(parsed_value)
    }

    pub async fn port_test(&mut self) -> Result<PortTest, Error> {
        let request = RpcRequest {
            method: Method::PortTest,
            arguments: None,
            tag: None,
        };
        let response = self.send_msg(&request).await?;
        let parsed_value = serde_json::from_value(value_from_response(response)?)?;
        Ok(parsed_value)
    }

    pub async fn free_space(&mut self, path: &str) -> Result<FreeSpace, Error> {
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
