//! Response types serialized back to the host.

/// A single sprite cut from the sheet, with its position and PNG bytes.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Sprite {
    pub row: u32,
    pub col: u32,
    /// Pixel offset of the sprite's left edge within the sheet.
    pub x: u32,
    /// Pixel offset of the sprite's top edge within the sheet.
    pub y: u32,
    pub width: u32,
    pub height: u32,
    /// PNG-encoded sprite. Emitted as a CBOR byte string so Typst decodes it
    /// straight to `bytes` (and the payload stays compact).
    #[serde(with = "serde_bytes")]
    pub png: Vec<u8>,
}

/// Response from [`split`](crate::split): the resolved grid plus every sprite.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SplitResponse {
    pub rows: u32,
    pub cols: u32,
    pub tile_width: u32,
    pub tile_height: u32,
    pub sprites: Vec<Sprite>,
}

/// Response from [`info`](crate::info): sheet dimensions and the resolved grid.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Info {
    pub sheet_width: u32,
    pub sheet_height: u32,
    pub rows: u32,
    pub cols: u32,
    pub tile_width: u32,
    pub tile_height: u32,
    pub count: u32,
}
