#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("NC error")]
    Nc(#[from] netc::error::Error),
    #[error("json error")]
    Json(#[from] serde_json::Error),
    #[error("dotenv error")]
    DotEnv(#[from] dotenv::Error),
    #[error("response not success: {0}")]
    BadResponse(String),
    #[error("response no contain arguments")]
    NoArguments,
    #[error("unmutable fields in session-set")]
    WrongSessionSetFields,
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("not auth")]
    NotAuth,
    #[error("TorrentAdd args have both filename and metadata")]
    BothFileMeta,
    #[error("TorrentAdd args no have filename or metadata")]
    NoFileMeta,
    #[error("Unknown torrent fields")]
    UnknownTorrentFields,
}
