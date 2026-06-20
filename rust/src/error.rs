//! Error type shared across the crate.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SpriteError {
    #[error("provide either `rows`+`cols` or `tile_width`+`tile_height`, not both")]
    AmbiguousSpec,

    #[error("provide a grid (`rows`+`cols`) or a tile size (`tile_width`+`tile_height`)")]
    MissingSpec,

    #[error("rows, columns, and tile sizes must all be greater than zero")]
    ZeroDimension,

    #[error("sprite count overflow")]
    SpriteCountOverflow,

    #[error(
        "a {rows}x{cols} grid with margin ({margin_x},{margin_y}) and spacing \
         ({spacing_x},{spacing_y}) does not fit in a {width}x{height} sheet"
    )]
    DoesNotFit {
        width: u32,
        height: u32,
        rows: u32,
        cols: u32,
        margin_x: u32,
        margin_y: u32,
        spacing_x: u32,
        spacing_y: u32,
    },

    #[error(
        "{axis} usable area {usable}px is not divisible by {count} tiles \
         (leftover {leftover}px); adjust margin/spacing or the grid"
    )]
    NonDivisibleGrid {
        axis: &'static str,
        usable: u32,
        count: u32,
        leftover: u32,
    },

    #[error("provide either `index` or both `row` and `col`")]
    MissingSelector,

    #[error("index {index} is out of range for {count} sprites")]
    IndexOutOfRange { index: u32, count: u32 },

    #[error("cell ({row},{col}) is out of range for a {rows}x{cols} grid")]
    CellOutOfRange {
        row: u32,
        col: u32,
        rows: u32,
        cols: u32,
    },

    #[error("image error: {0}")]
    Image(#[from] image::ImageError),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("CBOR error: {0}")]
    Cbor(String),
}
