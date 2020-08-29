use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::client::Client;
use crate::error::Result;
use crate::request::{Method, RpcRequest, IDS};
use crate::response::value_from_response;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TorrentFields {
    ActivityDate,
    AddedDate,
    BandwidthPriority,
    Comment,
    CorruptEver,
    Creator,
    DateCreated,
    DesiredAvailable,
    DoneDate,
    DownloadDir,
    DownloadedEver,
    DownloadLimit,
    DownloadLimited,
    EditDate,
    Error,
    ErrorString,
    Eta,
    EtaIdle,
    Files,
    FileStats,
    HashString,
    HaveUnchecked,
    HaveValid,
    HonorsSessionLimits,
    Id,
    IsFinished,
    IsPrivate,
    IsStalled,
    Labels,
    LeftUntilDone,
    MagnetLink,
    ManualAnnounceTime,
    MaxConnectedPeers,
    MetadataPercentComplete,
    Name,
    #[serde(rename = "peer-limit")]
    PeerLimit,
    Peers,
    PeersConnected,
    PeersFrom,
    PeersGettingFromUs,
    PeersSendingToUs,
    PercentDone,
    Pieces,
    PieceCount,
    PieceSize,
    Priorities,
    QueuePosition,
    RateDownload,
    RateUpload,
    RecheckProgress,
    SecondsDownloading,
    SecondsSeeding,
    SeedIdleLimit,
    SeedIdleMode,
    SeedRatioLimit,
    SeedRatioMode,
    SizeWhenDone,
    StartDate,
    Status,
    Trackers,
    TrackerStats,
    TotalSize,
    TorrentFile,
    UploadedEver,
    UploadLimit,
    UploadLimited,
    UploadRatio,
    Wanted,
    Webseeds,
    WebseedsSendingToUs,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub bytes_completed: i64,
    pub length: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileStats {
    pub bytes_completed: i64,
    pub wanted: bool,
    pub priority: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Peer {
    pub address: String,
    pub client_name: String,
    pub client_is_choked: bool,
    pub client_is_interested: bool,
    pub flag_str: String,
    pub is_downloading_from: bool,
    pub is_encrypted: bool,
    pub is_incoming: bool,
    pub is_uploading_to: bool,
    pub is_utp: bool,
    pub peer_is_choked: bool,
    pub peer_is_interested: bool,
    pub port: i64,
    pub progress: f64,
    pub rate_to_client: i64,
    pub rate_to_peer: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerFrom {
    pub from_cache: i64,
    pub from_incoming: i64,
    pub from_lpd: i64,
    pub from_ltep: i64,
    pub from_pex: i64,
    pub from_tracker: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tracker {
    pub announce: String,
    pub id: i64,
    pub scrape: String,
    pub tier: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackerStats {
    pub announce: String,
    pub announce_state: i64,
    pub download_count: i64,
    pub has_announced: bool,
    pub has_scraped: bool,
    pub host: String,
    pub id: i64,
    pub is_backup: bool,
    pub last_announce_peer_count: i64,
    pub last_announce_result: String,
    pub last_announce_start_time: i64,
    pub last_announce_succeeded: bool,
    pub last_announce_time: i64,
    pub last_announce_timed_out: bool,
    pub last_scrape_result: String,
    pub last_scrape_start_time: i64,
    pub last_scrape_succeeded: bool,
    pub last_scrape_time: i64,
    pub last_scrape_timed_out: bool,
    pub leecher_count: i64,
    pub next_announce_time: i64,
    pub next_scrape_time: i64,
    pub scrape: String,
    pub scrape_state: i64,
    pub seeder_count: i64,
    pub tier: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Webseed {
    pub webseed: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Torrent {
    pub activity_date: Option<i64>,
    pub added_date: Option<i64>,
    pub bandwidth_priority: Option<i64>,
    pub comment: Option<String>,
    pub corrupt_ever: Option<i64>,
    pub creator: Option<String>,
    pub date_created: Option<i64>,
    pub desired_available: Option<i64>,
    pub done_date: Option<i64>,
    pub download_dir: Option<String>,
    pub downloaded_ever: Option<i64>,
    pub download_limit: Option<i64>,
    pub download_limited: Option<bool>,
    pub edit_date: Option<i64>,
    pub error: Option<i64>,
    pub error_string: Option<String>,
    pub eta: Option<i64>,
    pub eta_idle: Option<i64>,
    pub files: Option<Vec<File>>,
    pub file_stats: Option<Vec<FileStats>>,
    pub hash_string: Option<String>,
    pub have_unchecked: Option<i64>,
    pub have_valid: Option<i64>,
    pub honors_session_limits: Option<bool>,
    pub id: Option<i64>,
    pub is_finished: Option<bool>,
    pub is_private: Option<bool>,
    pub is_stalled: Option<bool>,
    pub labels: Option<Vec<Label>>,
    pub left_until_done: Option<i64>,
    pub magnet_link: Option<String>,
    pub manual_announce_time: Option<i64>,
    pub max_connected_peers: Option<i64>,
    pub metadata_percent_complete: Option<f64>,
    pub name: Option<String>,
    #[serde(rename = "peer-limit")]
    pub peer_limit: Option<i64>,
    pub peers: Option<Vec<Peer>>,
    pub peers_connected: Option<i64>,
    pub peers_from: Option<Vec<PeerFrom>>,
    pub peers_getting_from_us: Option<i64>,
    pub peers_sending_to_us: Option<i64>,
    pub percent_done: Option<f64>,
    pub pieces: Option<String>,
    pub piece_count: Option<i64>,
    pub piece_size: Option<i64>,
    pub priorities: Option<Vec<i64>>,
    pub queue_position: Option<i64>,
    pub rate_download: Option<i64>,
    pub rate_upload: Option<i64>,
    pub recheck_progress: Option<f64>,
    pub seconds_downloading: Option<i64>,
    pub seconds_seeding: Option<i64>,
    pub seed_idle_limit: Option<i64>,
    pub seed_idle_mode: Option<i64>,
    pub seed_ratio_limit: Option<f64>,
    pub seed_ratio_mode: Option<i64>,
    pub size_when_done: Option<i64>,
    pub start_date: Option<i64>,
    pub status: Option<i64>,
    pub trackers: Option<Vec<Tracker>>,
    pub tracker_stats: Option<Vec<TrackerStats>>,
    pub total_size: Option<i64>,
    pub torrent_file: Option<String>,
    pub uploaded_ever: Option<i64>,
    pub upload_limit: Option<i64>,
    pub upload_limited: Option<bool>,
    pub upload_ratio: Option<f64>,
    pub wanted: Option<Vec<i64>>,
    pub webseeds: Option<Vec<Webseed>>,
    pub webseeds_sending_to_us: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TorrentGet {
  pub torrents: Vec<Torrent>,
}

impl Client {
    pub async fn torrent_get(
        &mut self,
        fields: Vec<TorrentFields>,
        ids: Option<IDS>,
    ) -> Result<TorrentGet> {
        let arguments = json!({"ids": ids,"fields": fields});
        let request = RpcRequest {
            method: Method::TorrentGet,
            arguments: Some(arguments),
            tag: None,
        };
        let response = self.send_msg(&request).await?;
        let parsed_value = serde_json::from_value(value_from_response(response)?)?;
        Ok(parsed_value)
    }
}
