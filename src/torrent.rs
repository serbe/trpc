use std::convert::TryFrom;
use std::io::Read;

use base64::encode_config_buf;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::client::Client;
use crate::error::Error;
use crate::request::{Ids, Method, RpcRequest};
use crate::response::value_from_response;

pub fn file_to_metadata(path: &str) -> Result<String, Error> {
    let mut file = std::fs::File::open(&path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let mut buf = String::new();
    encode_config_buf(buffer, base64::STANDARD, &mut buf);
    Ok(buf)
}

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
    #[serde(rename = "file-count")]
    FileCount,
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
    #[serde(rename = "primary-mime-type")]
    PrimaryMimeType,
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

impl TryFrom<&str> for TorrentFields {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Error> {
        match value.to_lowercase().as_str() {
            "activitydate" => Ok(TorrentFields::ActivityDate),
            "addeddate" => Ok(TorrentFields::AddedDate),
            "bandwidthpriority" => Ok(TorrentFields::BandwidthPriority),
            "comment" => Ok(TorrentFields::Comment),
            "corruptever" => Ok(TorrentFields::CorruptEver),
            "creator" => Ok(TorrentFields::Creator),
            "datecreated" => Ok(TorrentFields::DateCreated),
            "desiredavailable" => Ok(TorrentFields::DesiredAvailable),
            "donedate" => Ok(TorrentFields::DoneDate),
            "downloaddir" => Ok(TorrentFields::DownloadDir),
            "downloadedever" => Ok(TorrentFields::DownloadedEver),
            "downloadlimit" => Ok(TorrentFields::DownloadLimit),
            "downloadlimited" => Ok(TorrentFields::DownloadLimited),
            "editdate" => Ok(TorrentFields::EditDate),
            "error" => Ok(TorrentFields::Error),
            "errorstring" => Ok(TorrentFields::ErrorString),
            "eta" => Ok(TorrentFields::Eta),
            "etaidle" => Ok(TorrentFields::EtaIdle),
            "file-count" => Ok(TorrentFields::FileCount),
            "files" => Ok(TorrentFields::Files),
            "filestats" => Ok(TorrentFields::FileStats),
            "hashstring" => Ok(TorrentFields::HashString),
            "haveunchecked" => Ok(TorrentFields::HaveUnchecked),
            "havevalid" => Ok(TorrentFields::HaveValid),
            "honorssessionlimits" => Ok(TorrentFields::HonorsSessionLimits),
            "id" => Ok(TorrentFields::Id),
            "isfinished" => Ok(TorrentFields::IsFinished),
            "isprivate" => Ok(TorrentFields::IsPrivate),
            "isstalled" => Ok(TorrentFields::IsStalled),
            "labels" => Ok(TorrentFields::Labels),
            "leftuntildone" => Ok(TorrentFields::LeftUntilDone),
            "magnetlink" => Ok(TorrentFields::MagnetLink),
            "manualannouncetime" => Ok(TorrentFields::ManualAnnounceTime),
            "maxconnectedpeers" => Ok(TorrentFields::MaxConnectedPeers),
            "metadatapercentcomplete" => Ok(TorrentFields::MetadataPercentComplete),
            "name" => Ok(TorrentFields::Name),
            "peer-limit" => Ok(TorrentFields::PeerLimit),
            "peers" => Ok(TorrentFields::Peers),
            "peersconnected" => Ok(TorrentFields::PeersConnected),
            "peersfrom" => Ok(TorrentFields::PeersFrom),
            "peersgettingfromus" => Ok(TorrentFields::PeersGettingFromUs),
            "peerssendingtous" => Ok(TorrentFields::PeersSendingToUs),
            "percentdone" => Ok(TorrentFields::PercentDone),
            "pieces" => Ok(TorrentFields::Pieces),
            "piececount" => Ok(TorrentFields::PieceCount),
            "piecesize" => Ok(TorrentFields::PieceSize),
            "priorities" => Ok(TorrentFields::Priorities),
            "primary-mime-type" => Ok(TorrentFields::PrimaryMimeType),
            "queueposition" => Ok(TorrentFields::QueuePosition),
            "ratedownload" => Ok(TorrentFields::RateDownload),
            "rateupload" => Ok(TorrentFields::RateUpload),
            "recheckprogress" => Ok(TorrentFields::RecheckProgress),
            "secondsdownloading" => Ok(TorrentFields::SecondsDownloading),
            "secondsseeding" => Ok(TorrentFields::SecondsSeeding),
            "seedidlelimit" => Ok(TorrentFields::SeedIdleLimit),
            "seedidlemode" => Ok(TorrentFields::SeedIdleMode),
            "seedratiolimit" => Ok(TorrentFields::SeedRatioLimit),
            "seedratiomode" => Ok(TorrentFields::SeedRatioMode),
            "sizewhendone" => Ok(TorrentFields::SizeWhenDone),
            "startdate" => Ok(TorrentFields::StartDate),
            "status" => Ok(TorrentFields::Status),
            "trackers" => Ok(TorrentFields::Trackers),
            "trackerstats" => Ok(TorrentFields::TrackerStats),
            "totalsize" => Ok(TorrentFields::TotalSize),
            "torrentfile" => Ok(TorrentFields::TorrentFile),
            "uploadedever" => Ok(TorrentFields::UploadedEver),
            "uploadlimit" => Ok(TorrentFields::UploadLimit),
            "uploadlimited" => Ok(TorrentFields::UploadLimited),
            "uploadratio" => Ok(TorrentFields::UploadRatio),
            "wanted" => Ok(TorrentFields::Wanted),
            "webseeds" => Ok(TorrentFields::Webseeds),
            "webseedssendingtous" => Ok(TorrentFields::WebseedsSendingToUs),
            _ => Err(Error::UnknownTorrentFields),
        }
    }
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

#[derive(Debug, Serialize, Deserialize)]
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
    #[serde(rename = "file-count")]
    pub file_count: Option<i64>,
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
    #[serde(rename = "primary-mime-type")]
    pub primary_mime_type: Option<String>,
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
#[serde(rename_all = "camelCase")]
pub struct TorrentAddResponse {
    pub hash_string: String,
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TorrentSetArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth_priority: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_limited: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "files-wanted")]
    pub files_wanted: Option<File>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "files-unwanted")]
    pub files_unwanted: Option<File>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honors_session_limits: Option<bool>,
    pub ids: Ids,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "peer-limit")]
    pub peer_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "priority-high")]
    pub priority_high: Option<File>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "priority-low")]
    pub priority_low: Option<File>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "priority-normal")]
    pub priority_normal: Option<File>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_position: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_idle_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_idle_mode: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_ratio_limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_ratio_mode: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracker_add: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracker_remove: Option<Ids>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracker_replace: Option<Ids>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_limited: Option<bool>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TorrentGetArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Ids>,
    pub fields: Vec<TorrentFields>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TorrentAddArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metainfo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bandwidthPriority")]
    pub bandwidth_priority: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_wanted: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_unwanted: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_high: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_low: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_normal: Option<i64>,
}

impl TorrentAddArgs {
    pub fn from_meta(path: &str) -> Result<Self, Error> {
        let metainfo = Some(file_to_metadata(path)?);
        Ok(TorrentAddArgs {
            cookies: None,
            download_dir: None,
            filename: None,
            metainfo,
            paused: None,
            peer_limit: None,
            bandwidth_priority: None,
            files_wanted: None,
            files_unwanted: None,
            priority_high: None,
            priority_low: None,
            priority_normal: None,
        })
    }

    pub fn from_file(path: &str) -> Result<Self, Error> {
        Ok(TorrentAddArgs {
            cookies: None,
            download_dir: None,
            filename: Some(path.to_string()),
            metainfo: None,
            paused: None,
            peer_limit: None,
            bandwidth_priority: None,
            files_wanted: None,
            files_unwanted: None,
            priority_high: None,
            priority_low: None,
            priority_normal: None,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TorrentRemoveArgs {
    pub ids: Ids,
    pub delete_local_data: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TorrentSetLocationArgs {
    pub ids: Ids,
    pub location: String,
    #[serde(rename = "move")]
    pub move_local_data: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TorrentRenamePathArgs {
    pub ids: Ids,
    pub path: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TorrentGet {
    pub torrents: Vec<Torrent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TorrentAdd {
    pub torrent: Option<TorrentAddResponse>,
    #[serde(rename = "torrent-duplicate")]
    pub torrent_duplicate: Option<TorrentAddResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TorrentRenamePath {
    pub id: i64,
    pub name: String,
    pub path: String,
}

impl Client {
    pub async fn torrent_start(&mut self, args: Option<Ids>) -> Result<(), Error> {
        let value = args.map(|args| json!(args));
        let request = RpcRequest {
            method: Method::TorrentStart,
            arguments: value,
            tag: None,
        };
        let response = self.send_msg(&request).await?;
        let _ = value_from_response(response)?;
        Ok(())
    }

    pub async fn torrent_start_now(&mut self, args: Option<Ids>) -> Result<(), Error> {
        let value = args.map(|args| json!(args));
        let request = RpcRequest {
            method: Method::TorrentStartNow,
            arguments: value,
            tag: None,
        };
        let response = self.send_msg(&request).await?;
        let _ = value_from_response(response)?;
        Ok(())
    }

    pub async fn torrent_stop(&mut self, args: Option<Ids>) -> Result<(), Error> {
        let value = args.map(|args| json!(args));
        let request = RpcRequest {
            method: Method::TorrentStop,
            arguments: value,
            tag: None,
        };
        let response = self.send_msg(&request).await?;
        let _ = value_from_response(response)?;
        Ok(())
    }

    pub async fn torrent_verify(&mut self, args: Option<Ids>) -> Result<(), Error> {
        let value = args.map(|args| json!(args));
        let request = RpcRequest {
            method: Method::TorrentVerify,
            arguments: value,
            tag: None,
        };
        let response = self.send_msg(&request).await?;
        let _ = value_from_response(response)?;
        Ok(())
    }

    pub async fn torrent_reannounce(&mut self, args: Option<Ids>) -> Result<(), Error> {
        let value = args.map(|args| json!(args));
        let request = RpcRequest {
            method: Method::TorrentReannounce,
            arguments: value,
            tag: None,
        };
        let response = self.send_msg(&request).await?;
        let _ = value_from_response(response)?;
        Ok(())
    }

    pub async fn torrent_set(&mut self, args: TorrentSetArgs) -> Result<(), Error> {
        let request = RpcRequest {
            method: Method::TorrentSet,
            arguments: Some(json!(args)),
            tag: None,
        };
        let response = self.send_msg(&request).await?;
        let _ = value_from_response(response)?;
        Ok(())
    }

    pub async fn torrent_get(&mut self, args: TorrentGetArgs) -> Result<TorrentGet, Error> {
        let request = RpcRequest {
            method: Method::TorrentGet,
            arguments: Some(json!(args)),
            tag: None,
        };
        let response = self.send_msg(&request).await?;
        let parsed_value = serde_json::from_value(value_from_response(response)?)?;
        Ok(parsed_value)
    }

    pub async fn torrent_add(&mut self, args: TorrentAddArgs) -> Result<TorrentAdd, Error> {
        match (&args.filename, &args.metainfo) {
            (Some(_), Some(_)) => Err(Error::BothFileMeta),
            (None, None) => Err(Error::NoFileMeta),
            _ => Ok(()),
        }?;
        let request = RpcRequest {
            method: Method::TorrentAdd,
            arguments: Some(json!(args)),
            tag: None,
        };
        let response = self.send_msg(&request).await?;
        let parsed_value = serde_json::from_value(value_from_response(response)?)?;
        Ok(parsed_value)
    }

    pub async fn torrent_remove(&mut self, args: TorrentRemoveArgs) -> Result<(), Error> {
        let request = RpcRequest {
            method: Method::TorrentRemove,
            arguments: Some(json!(args)),
            tag: None,
        };
        let response = self.send_msg(&request).await?;
        let _ = value_from_response(response)?;
        Ok(())
    }

    pub async fn torrent_set_location(
        &mut self,
        args: TorrentSetLocationArgs,
    ) -> Result<(), Error> {
        let request = RpcRequest {
            method: Method::TorrentSetLocation,
            arguments: Some(json!(args)),
            tag: None,
        };
        let response = self.send_msg(&request).await?;
        let _ = value_from_response(response)?;
        Ok(())
    }

    pub async fn torrent_rename_path(
        &mut self,
        args: TorrentRenamePathArgs,
    ) -> Result<TorrentRenamePath, Error> {
        let request = RpcRequest {
            method: Method::TorrentAdd,
            arguments: Some(json!(args)),
            tag: None,
        };
        let response = self.send_msg(&request).await?;
        let parsed_value = serde_json::from_value(value_from_response(response)?)?;
        Ok(parsed_value)
    }
}
