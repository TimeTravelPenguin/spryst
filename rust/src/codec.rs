//! CBOR (de)serialization and image decoding helpers.

use std::io::Cursor;

use ciborium::{de::from_reader, ser::into_writer};
use image::{DynamicImage, ImageReader};

use crate::error::SpriteError;

pub fn decode<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T, SpriteError> {
    from_reader(Cursor::new(bytes)).map_err(|err| SpriteError::Cbor(err.to_string()))
}

pub fn encode<T: serde::Serialize>(value: &T) -> Result<Vec<u8>, SpriteError> {
    let mut out = Vec::new();
    into_writer(value, &mut out).map_err(|err| SpriteError::Cbor(err.to_string()))?;

    Ok(out)
}

pub fn decode_image(bytes: &[u8]) -> Result<DynamicImage, SpriteError> {
    Ok(ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?)
}
