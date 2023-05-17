use std::{io, str::Utf8Error};

#[cfg(feature = "pack")]
use rectangle_pack::RectanglePackError;
use thiserror::Error;

///
#[derive(Error, Debug)]
pub enum PxoError {
    ///
    #[error("failed to read from file")]
    ReadFile(#[from] io::Error),
    ///
    #[error("failed to read utf8 bytes")]
    ReadUtf8(#[from] Utf8Error),
    ///
    #[error("failed to read json")]
    ReadJson(#[from] serde_json::Error),
    ///
    #[error("unexpected json value")]
    UnexpectedJson,
    ///
    #[error("expected godot magic header '{0}' got '{1}'")]
    UnexpectedMagic(String, String),
    ///
    #[error("expected compression mode 2 (zltf), got {0}")]
    UnexpectedCompressionMode(u32),
    ///
    #[error("block size cannot be zero")]
    ZeroBlockSize,
    ///
    #[error("failed to read image")]
    ReadImage,
    ///
    #[error("failed to convert raw pxo to sprite")]
    SpriteConversion,
    ///
    #[error("failed to pack sprite(s)")]
    RectanglePack,
}

#[cfg(feature = "pack")]
impl From<RectanglePackError> for PxoError {
    fn from(_: RectanglePackError) -> Self {
        PxoError::RectanglePack
    }
}
