use std::convert::From;

use serde::{Deserialize, Serialize, Serializer};
use serde_json::{json, Value};

use crate::client::Client;
use crate::error::Error;
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
    QueueMoveTop,
    QueueMoveUp,
    QueueMoveDown,
    QueueMoveBottom,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Id {
    Id(i64),
    Hash(String),
}

impl From<i64> for Id {
    fn from(id: i64) -> Self {
        Id::Id(id)
    }
}

impl From<&str> for Id {
    fn from(hash: &str) -> Self {
        Id::Hash(hash.to_string())
    }
}

impl From<String> for Id {
    fn from(hash: String) -> Self {
        Id::Hash(hash)
    }
}

#[derive(Debug, Deserialize)]
pub enum Ids {
    Id(i64),
    Array(Vec<Id>),
    RecentlyActive,
}

impl Default for Ids {
    fn default() -> Self {
        Ids::RecentlyActive
    }
}

impl From<i64> for Ids {
    fn from(id: i64) -> Self {
        Ids::Id(id)
    }
}

impl From<Vec<Id>> for Ids {
    fn from(values: Vec<Id>) -> Self {
        let mut arr = Vec::new();
        for value in values {
            arr.push(value);
        }
        Ids::Array(arr)
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
            Ids::RecentlyActive => serializer.serialize_str("recently-active"),
        }
    }
}

pub fn value_from_response(response: RpcResponse) -> Result<Value, Error> {
    if &response.result == "success" {
        Ok(response.arguments.map_or(Err(Error::NoArguments), Ok)?)
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

    pub async fn queue_move_top(&mut self, args: Option<Ids>) -> Result<(), Error> {
        let value = args.map(|args| json!(args));
        let request = RpcRequest {
            method: Method::QueueMoveTop,
            arguments: value,
            tag: None,
        };
        let response = self.send_msg(&request).await?;
        let _ = value_from_response(response)?;
        Ok(())
    }

    pub async fn queue_move_up(&mut self, args: Option<Ids>) -> Result<(), Error> {
        let value = args.map(|args| json!(args));
        let request = RpcRequest {
            method: Method::QueueMoveUp,
            arguments: value,
            tag: None,
        };
        let response = self.send_msg(&request).await?;
        let _ = value_from_response(response)?;
        Ok(())
    }

    pub async fn queue_move_down(&mut self, args: Option<Ids>) -> Result<(), Error> {
        let value = args.map(|args| json!(args));
        let request = RpcRequest {
            method: Method::QueueMoveDown,
            arguments: value,
            tag: None,
        };
        let response = self.send_msg(&request).await?;
        let _ = value_from_response(response)?;
        Ok(())
    }

    pub async fn queue_move_bottom(&mut self, args: Option<Ids>) -> Result<(), Error> {
        let value = args.map(|args| json!(args));
        let request = RpcRequest {
            method: Method::QueueMoveBottom,
            arguments: value,
            tag: None,
        };
        let response = self.send_msg(&request).await?;
        let _ = value_from_response(response)?;
        Ok(())
    }
}
