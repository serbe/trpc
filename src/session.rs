use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::Result;
use crate::request::{Method, RpcRequest};
use crate::response::value_from_response;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub uploaded_bytes: i64,
    pub downloaded_bytes: i64,
    pub files_added: i64,
    pub session_count: i64,
    pub seconds_active: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionStats {
    pub active_torrent_count: i64,
    pub download_speed: i64,
    pub paused_torrent_count: i64,
    pub torrent_count: i64,
    pub upload_speed: i64,
    #[serde(rename = "cumulative-stats")]
    pub cumulative_stats: Stats,
    #[serde(rename = "current-stats")]
    pub current_stats: Stats,
}

impl Client {
    pub async fn session_stats(&mut self) -> Result<SessionStats> {
        let request = RpcRequest {
            method: Method::SessionStats,
            arguments: None,
            tag: None,
        };
        let response = self.send_msg(&request).await?;
        let parsed_value = serde_json::from_value(value_from_response(response)?)?;
        Ok(parsed_value)
    }

    pub async fn session_close(&mut self) -> Result<()> {
        let request = RpcRequest {
            method: Method::SessionClose,
            arguments: None,
            tag: None,
        };
        let response = self.send_msg(&request).await?;
        let _ = value_from_response(response)?;
        Ok(())
    }
}
