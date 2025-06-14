use serde::Serialize;
use thiserror::Error;

/// Possible errors during requesting/decrypting/decoding/deserialization e.g.
#[derive(Debug, Error)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum Error {
    #[error("key length not match! expected 32, got {0}")]
    KeyLengthMismatch(usize),

    #[error("iterations must be non zero!")]
    ZeroIterations,

    #[error("url must be like https://paste.fitgirl-repacks.site/?{{pasteid}}#{{key_base58}}")]
    IllFormedURL,

    #[cfg(feature = "ureq")]
    #[error("request error: {0}")]
    Ureq(String),

    #[cfg(feature = "reqwest")]
    #[error("request error: {0}")]
    Reqwest(String),

    #[cfg(feature = "nyquest")]
    #[error("request error: {0}")]
    Nyquest(String),

    #[cfg(feature = "nyquest")]
    #[error("build client error: {0}")]
    NyquestBuildClient(String),

    #[error("base58 decode error: {0}")]
    Base58(String),

    #[error("base64 decode error: {0}")]
    Base64(String),

    #[error("zlib decompress error")]
    DecompressError,

    #[error("aes-256-gcm decryption error")]
    AesGcm,

    #[error("deserialize error: {0}")]
    JSONSerialize(String),
}

#[cfg(feature = "ureq")]
impl From<ureq::Error> for Error {
    fn from(err: ureq::Error) -> Self {
        Error::Ureq(err.to_string())
    }
}

#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err.to_string())
    }
}

#[cfg(feature = "nyquest")]
impl From<nyquest::Error> for Error {
    fn from(err: nyquest::Error) -> Self {
        Error::Nyquest(err.to_string())
    }
}

#[cfg(feature = "nyquest")]
impl From<nyquest::client::BuildClientError> for Error {
    fn from(err: nyquest::client::BuildClientError) -> Self {
        Error::NyquestBuildClient(err.to_string())
    }
}

impl From<bs58::decode::Error> for Error {
    fn from(err: bs58::decode::Error) -> Self {
        Error::Base58(err.to_string())
    }
}

impl From<base64::DecodeError> for Error {
    fn from(err: base64::DecodeError) -> Self {
        Error::Base64(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JSONSerialize(err.to_string())
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
