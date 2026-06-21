//! # spryst
//!
//! A [Typst](https://typst.app) WASM plugin for slicing a spritesheet into its
//! individual sprites.
//!
//! The plugin exposes three functions, each taking the raw spritesheet bytes
//! plus a [CBOR](https://cbor.io)-encoded request and returning a CBOR-encoded
//! response:
//!
//! - [`split`] — decode the sheet and return every sprite as a PNG.
//! - [`sprite`] — return a single sprite, addressed by index or `(row, col)`.
//! - [`info`] — report the sheet dimensions and the resolved grid, without
//!   encoding any sprites.
//!
//! ## Slicing model
//!
//! A sheet is described by a [`SliceSpec`]. The grid can be given in one of two
//! ways:
//!
//! - **grid mode** — `rows` and `cols`; the tile size is derived and must divide
//!   the usable area evenly.
//! - **size mode** — `tile_width` and `tile_height`; the row/column counts are
//!   derived as the number of whole tiles that fit.
//!
//! Both modes honour an optional `margin` (a border between the sheet edges and
//! the outermost tiles) and `spacing` (the gap between adjacent tiles), each
//! configurable per axis. This matches the layout used by common tileset tools.
//!
//! On error every function returns `Err`, whose message is surfaced by Typst as
//! a diagnostic.

mod codec;
mod error;
mod layout;
mod model;
mod spec;

pub use error::SpriteError;
pub use model::{Info, Sprite, SplitResponse};
pub use spec::{Selector, SliceSpec};

use codec::{decode, decode_image, encode};
use layout::Layout;

#[cfg(target_arch = "wasm32")]
use wasm_minimal_protocol::{initiate_protocol, wasm_func};

#[cfg(target_arch = "wasm32")]
initiate_protocol!();

/// Decode `sheet` and return every sprite as a PNG, per the CBOR [`SliceSpec`]
/// in `request`.
#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn split(sheet: &[u8], request: &[u8]) -> Result<Vec<u8>, SpriteError> {
    let spec: SliceSpec = decode(request)?;

    let img = decode_image(sheet)?;
    let (width, height) = image::GenericImageView::dimensions(&img);
    let layout = Layout::resolve(&spec, width, height)?;

    let mut sprites = Vec::with_capacity(layout.count()? as usize);

    for row in 0..layout.rows {
        for col in 0..layout.cols {
            sprites.push(layout.cut(&img, row, col)?);
        }
    }

    encode(&SplitResponse {
        rows: layout.rows,
        cols: layout.cols,
        tile_width: layout.tile_w,
        tile_height: layout.tile_h,
        sprites,
    })
}

/// Decode `sheet` and return the single sprite chosen by the CBOR [`Selector`]
/// in `selector`, using the [`SliceSpec`] in `spec`.
#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn sprite(sheet: &[u8], spec: &[u8], selector: &[u8]) -> Result<Vec<u8>, SpriteError> {
    let spec: SliceSpec = decode(spec)?;
    let selector: Selector = decode(selector)?;

    let img = decode_image(sheet)?;
    let (width, height) = image::GenericImageView::dimensions(&img);
    let layout = Layout::resolve(&spec, width, height)?;

    let (row, col) = layout.locate(&selector)?;

    encode(&layout.cut(&img, row, col)?)
}

/// Decode just enough of `sheet` to report its dimensions and the grid resolved
/// from the CBOR [`SliceSpec`] in `request`. No sprites are encoded.
#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn info(sheet: &[u8], request: &[u8]) -> Result<Vec<u8>, SpriteError> {
    let spec: SliceSpec = decode(request)?;

    let img = decode_image(sheet)?;
    let (width, height) = image::GenericImageView::dimensions(&img);
    let layout = Layout::resolve(&spec, width, height)?;

    encode(&Info {
        sheet_width: width,
        sheet_height: height,
        rows: layout.rows,
        cols: layout.cols,
        tile_width: layout.tile_w,
        tile_height: layout.tile_h,
        count: layout.count()?,
    })
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use image::{GenericImageView, ImageFormat};

    use super::*;

    /// A `cols`x`rows` checkerboard-ish PNG, each cell `tile` pixels square.
    fn sheet(cols: u32, rows: u32, tile: u32) -> Vec<u8> {
        let mut img = image::RgbaImage::new(cols * tile, rows * tile);

        for (px, py, pixel) in img.enumerate_pixels_mut() {
            let cell = (px / tile + py / tile) % 2;
            *pixel = if cell == 0 {
                image::Rgba([255, 0, 0, 255])
            } else {
                image::Rgba([0, 0, 255, 255])
            };
        }

        let mut bytes = Vec::new();
        image::DynamicImage::ImageRgba8(img)
            .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
            .unwrap();

        bytes
    }

    fn grid_spec(rows: u32, cols: u32) -> SliceSpec {
        SliceSpec {
            rows: Some(rows),
            cols: Some(cols),
            ..Default::default()
        }
    }

    #[test]
    fn resolves_grid_mode() {
        let bytes = sheet(4, 2, 16);
        let img = decode_image(&bytes).unwrap();
        let (w, h) = img.dimensions();

        let layout = Layout::resolve(&grid_spec(2, 4), w, h).unwrap();

        assert_eq!((layout.rows, layout.cols), (2, 4));
        assert_eq!((layout.tile_w, layout.tile_h), (16, 16));
        assert_eq!(layout.count().unwrap(), 8);
    }

    #[test]
    fn resolves_size_mode_with_floor() {
        let bytes = sheet(4, 2, 16); // 64x32
        let img = decode_image(&bytes).unwrap();
        let (w, h) = img.dimensions();

        let spec = SliceSpec {
            tile_width: Some(16),
            tile_height: Some(16),
            ..Default::default()
        };

        let layout = Layout::resolve(&spec, w, h).unwrap();
        assert_eq!((layout.rows, layout.cols), (2, 4));
    }

    #[test]
    fn honours_margin_and_spacing_in_grid_mode() {
        // 2 cols of 10px, 1 spacing between, 3 margin each side = 6 + 20 + 1 = 27 wide.
        let mut img = image::RgbaImage::new(27, 27);
        for pixel in img.pixels_mut() {
            *pixel = image::Rgba([0, 0, 0, 255]);
        }

        let mut bytes = Vec::new();
        image::DynamicImage::ImageRgba8(img)
            .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
            .unwrap();

        let spec = SliceSpec {
            rows: Some(2),
            cols: Some(2),
            margin_x: 3,
            margin_y: 3,
            spacing_x: 1,
            spacing_y: 1,
            ..Default::default()
        };

        let img = decode_image(&bytes).unwrap();
        let (w, h) = img.dimensions();
        let layout = Layout::resolve(&spec, w, h).unwrap();

        assert_eq!((layout.tile_w, layout.tile_h), (10, 10));
        assert_eq!(layout.origin(0, 0), (3, 3));
        assert_eq!(layout.origin(1, 1), (3 + 10 + 1, 3 + 10 + 1));
    }

    #[test]
    fn rejects_ambiguous_and_missing_specs() {
        let ambiguous = SliceSpec {
            rows: Some(1),
            cols: Some(1),
            tile_width: Some(1),
            tile_height: Some(1),
            ..Default::default()
        };

        assert!(matches!(
            Layout::resolve(&ambiguous, 16, 16),
            Err(SpriteError::AmbiguousSpec)
        ));

        assert!(matches!(
            Layout::resolve(&SliceSpec::default(), 16, 16),
            Err(SpriteError::MissingSpec)
        ));
    }

    #[test]
    fn rejects_non_divisible_grid() {
        let bytes = sheet(4, 2, 16); // 64x32
        let img = decode_image(&bytes).unwrap();
        let (w, h) = img.dimensions();

        // 64 is not divisible by 3.
        assert!(matches!(
            Layout::resolve(&grid_spec(2, 3), w, h),
            Err(SpriteError::NonDivisibleGrid { .. })
        ));
    }

    #[test]
    fn split_returns_every_sprite() {
        let bytes = sheet(4, 2, 16);
        let request = encode(&grid_spec(2, 4)).unwrap();

        let response = split(&bytes, &request).unwrap();
        let decoded: SplitResponse = {
            // Round-trip the response through CBOR to confirm it is well-formed.
            let value: ciborium::value::Value = decode(&response).unwrap();
            value.deserialized().unwrap()
        };

        assert_eq!((decoded.rows, decoded.cols), (2, 4));
        assert_eq!(decoded.sprites.len(), 8);
        assert_eq!(decoded.sprites[5].row, 1);
        assert_eq!(decoded.sprites[5].col, 1);
        assert!(!decoded.sprites[0].png.is_empty());
    }

    #[test]
    fn sprite_by_index_and_cell_agree() {
        let bytes = sheet(4, 2, 16);
        let spec = encode(&grid_spec(2, 4)).unwrap();

        let by_index = sprite(
            &bytes,
            &spec,
            &encode(&Selector {
                index: Some(5),
                ..Default::default()
            })
            .unwrap(),
        )
        .unwrap();

        // index 5 in a 4-col grid is row 1, col 1.
        let by_cell = sprite(
            &bytes,
            &spec,
            &encode(&Selector {
                row: Some(1),
                col: Some(1),
                ..Default::default()
            })
            .unwrap(),
        )
        .unwrap();

        assert_eq!(by_index, by_cell);
    }

    #[test]
    fn sprite_out_of_range_errors() {
        let bytes = sheet(4, 2, 16);
        let spec = encode(&grid_spec(2, 4)).unwrap();

        let err = sprite(
            &bytes,
            &spec,
            &encode(&Selector {
                index: Some(99),
                ..Default::default()
            })
            .unwrap(),
        );

        assert!(matches!(err, Err(SpriteError::IndexOutOfRange { .. })));
    }
}
