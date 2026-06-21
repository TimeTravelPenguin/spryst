//! Request types: how the caller describes the slice and selects a sprite.

/// How to carve a sheet into sprites.
///
/// Provide *either* `rows` + `cols` (grid mode) *or* `tile_width` +
/// `tile_height` (size mode). Supplying both pairs, or neither, is an error.
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SliceSpec {
    /// Number of sprite rows (grid mode).
    #[serde(default)]
    pub rows: Option<u32>,

    /// Number of sprite columns (grid mode).
    #[serde(default)]
    pub cols: Option<u32>,

    /// Width of a single sprite in pixels (size mode).
    #[serde(default)]
    pub tile_width: Option<u32>,

    /// Height of a single sprite in pixels (size mode).
    #[serde(default)]
    pub tile_height: Option<u32>,

    /// Border between the left/right edges and the outermost tiles, in pixels.
    #[serde(default)]
    pub margin_x: u32,

    /// Border between the top/bottom edges and the outermost tiles, in pixels.
    #[serde(default)]
    pub margin_y: u32,

    /// Horizontal gap between adjacent tiles, in pixels.
    #[serde(default)]
    pub spacing_x: u32,

    /// Vertical gap between adjacent tiles, in pixels.
    #[serde(default)]
    pub spacing_y: u32,
}

/// Which sprite [`sprite`](crate::sprite) should return.
///
/// Provide either `index` (row-major, zero-based) or both `row` and `col`.
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Selector {
    /// Row-major, zero-based index into the grid.
    #[serde(default)]
    pub index: Option<u32>,

    /// Zero-based row.
    #[serde(default)]
    pub row: Option<u32>,

    /// Zero-based column.
    #[serde(default)]
    pub col: Option<u32>,
}
