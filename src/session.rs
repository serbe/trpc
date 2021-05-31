use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::client::Client;
use crate::error::Error;
use crate::request::{Method, RpcRequest};
use crate::response::value_from_response;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SessionFields {
    AltSpeedDown,
    AltSpeedEnabled,
    AltSpeedTimeBegin,
    AltSpeedTimeEnabled,
    AltSpeedTimeEnd,
    AltSpeedTimeDay,
    AltSpeedUp,
    BlocklistUrl,
    BlocklistEnabled,
    BlocklistSize,
    CacheSizeMb,
    ConfigDir,
    DownloadDir,
    DownloadQueueSize,
    DownloadQueueEnabled,
    DhtEnabled,
    Encryption,
    IdleSeedingLimit,
    IdleSeedingLimitEnabled,
    IncompleteDir,
    IncompleteDirEnabled,
    LpdEnabled,
    PeerLimitGlobal,
    PeerLimitPerTorrent,
    PexEnabled,
    PeerPort,
    PeerPortRandomOnStart,
    PortForwardingEnabled,
    QueueStalledEnabled,
    QueueStalledMinutes,
    RenamePartialFiles,
    RpcVersion,
    RpcVersionMinimum,
    ScriptTorrentDoneFilename,
    ScriptTorrentDoneEnabled,
    #[serde(rename = "seedRatioLimit")]
    SeedRatioLimit,
    #[serde(rename = "seedRatioLimited")]
    SeedRatioLimited,
    SeedQueueSize,
    SeedQueueEnabled,
    SessionID,
    SpeedLimitDown,
    SpeedLimitDownEnabled,
    SpeedLimitUp,
    SpeedLimitUpEnabled,
    StartAddedTorrents,
    TrashOriginalTorrentFiles,
    Units,
    UtpEnabled,
    Version,
}

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Units {
    pub speed_units: Vec<String>,
    pub speed_bytes: i64,
    pub size_units: Vec<String>,
    pub size_bytes: i64,
    pub memory_units: Vec<String>,
    pub memory_bytes: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Session {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_speed_down: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_speed_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_speed_time_begin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_speed_time_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_speed_time_end: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_speed_time_day: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_speed_up: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocklist_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocklist_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocklist_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_size_mb: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_queue_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_queue_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dht_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_seeding_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_seeding_limit_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_dir_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lpd_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_limit_global: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_limit_per_torrent: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pex_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_port: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_port_random_on_start: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_forwarding_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_stalled_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_stalled_minutes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rename_partial_files: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpc_version: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpc_version_minimum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_torrent_done_filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_torrent_done_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "seedRatioLimit")]
    pub seed_ratio_limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "seedRatioLimited")]
    pub seed_ratio_limited: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_queue_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_queue_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_limit_down: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_limit_down_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_limit_up: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_limit_up_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_added_torrents: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trash_original_torrent_files: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<Units>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utp_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<bool>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SessionGetArgs {
    pub fields: Vec<SessionFields>,
}

impl Client {
    pub async fn session_set(&mut self, args: Session) -> Result<(), Error> {
        if args.blocklist_size.is_some()
            || args.config_dir.is_some()
            || args.rpc_version.is_some()
            || args.rpc_version_minimum.is_some()
            || args.version.is_some()
            || args.session_id.is_some()
        {
            return Err(Error::WrongSessionSetFields);
        }
        let request = RpcRequest {
            method: Method::SessionGet,
            arguments: Some(json!(args)),
            tag: None,
        };
        let response = self.send_msg(&request).await?;
        let _ = value_from_response(response)?;
        Ok(())
    }

    pub async fn session_get(&mut self, args: Option<SessionGetArgs>) -> Result<Session, Error> {
        let value = args.map(|args| json!(args));
        let request = RpcRequest {
            method: Method::SessionGet,
            arguments: value,
            tag: None,
        };
        let response = self.send_msg(&request).await?;
        let parsed_value = serde_json::from_value(value_from_response(response)?)?;
        Ok(parsed_value)
    }

    pub async fn session_stats(&mut self) -> Result<SessionStats, Error> {
        let request = RpcRequest {
            method: Method::SessionStats,
            arguments: None,
            tag: None,
        };
        let response = self.send_msg(&request).await?;
        let parsed_value = serde_json::from_value(value_from_response(response)?)?;
        Ok(parsed_value)
    }

    pub async fn session_close(&mut self) -> Result<(), Error> {
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
