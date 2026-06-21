//! Resolving a [`SliceSpec`] into concrete tile geometry, and cutting sprites.

use std::io::Cursor;

use image::{DynamicImage, ImageFormat};

use crate::error::SpriteError;
use crate::model::Sprite;
use crate::spec::{Selector, SliceSpec};

/// A fully resolved grid: concrete tile sizes and offsets for the sheet.
#[derive(Debug, Clone, Copy)]
pub struct Layout {
    pub rows: u32,
    pub cols: u32,
    pub tile_w: u32,
    pub tile_h: u32,
    pub margin_x: u32,
    pub margin_y: u32,
    pub spacing_x: u32,
    pub spacing_y: u32,
}

impl Layout {
    /// Resolve a [`SliceSpec`] against the sheet's pixel dimensions.
    pub fn resolve(spec: &SliceSpec, width: u32, height: u32) -> Result<Self, SpriteError> {
        let has_grid = spec.rows.is_some() || spec.cols.is_some();
        let has_tiles = spec.tile_width.is_some() || spec.tile_height.is_some();

        let (rows, cols, tile_w, tile_h) = match (has_grid, has_tiles) {
            (true, true) => return Err(SpriteError::AmbiguousSpec),
            (false, false) => return Err(SpriteError::MissingSpec),
            (true, false) => {
                let rows = spec.rows.ok_or(SpriteError::MissingSpec)?;
                let cols = spec.cols.ok_or(SpriteError::MissingSpec)?;

                let tile_w = tile_size("horizontal", width, cols, spec.margin_x, spec.spacing_x)?;
                let tile_h = tile_size("vertical", height, rows, spec.margin_y, spec.spacing_y)?;

                (rows, cols, tile_w, tile_h)
            }
            (false, true) => {
                let tile_w = spec.tile_width.ok_or(SpriteError::MissingSpec)?;
                let tile_h = spec.tile_height.ok_or(SpriteError::MissingSpec)?;

                let cols = tile_fit(width, tile_w, spec.margin_x, spec.spacing_x)?;
                let rows = tile_fit(height, tile_h, spec.margin_y, spec.spacing_y)?;

                (rows, cols, tile_w, tile_h)
            }
        };

        if rows == 0 || cols == 0 || tile_w == 0 || tile_h == 0 {
            return Err(SpriteError::ZeroDimension);
        }

        let layout = Layout {
            rows,
            cols,
            tile_w,
            tile_h,
            margin_x: spec.margin_x,
            margin_y: spec.margin_y,
            spacing_x: spec.spacing_x,
            spacing_y: spec.spacing_y,
        };

        layout.ensure_fits(width, height)?;

        Ok(layout)
    }

    pub fn count(&self) -> Result<u32, SpriteError> {
        self.rows
            .checked_mul(self.cols)
            .ok_or(SpriteError::SpriteCountOverflow)
    }

    /// Top-left pixel offset of the cell at `(row, col)`.
    pub fn origin(&self, row: u32, col: u32) -> (u32, u32) {
        let x = self.margin_x + col * (self.tile_w + self.spacing_x);
        let y = self.margin_y + row * (self.tile_h + self.spacing_y);

        (x, y)
    }

    /// Translate a [`Selector`] into a `(row, col)` cell, bounds-checked.
    pub fn locate(&self, selector: &Selector) -> Result<(u32, u32), SpriteError> {
        if let Some(index) = selector.index {
            let count = self.count()?;

            if index >= count {
                return Err(SpriteError::IndexOutOfRange { index, count });
            }

            return Ok((index / self.cols, index % self.cols));
        }

        match (selector.row, selector.col) {
            (Some(row), Some(col)) => {
                if row >= self.rows || col >= self.cols {
                    return Err(SpriteError::CellOutOfRange {
                        row,
                        col,
                        rows: self.rows,
                        cols: self.cols,
                    });
                }

                Ok((row, col))
            }
            _ => Err(SpriteError::MissingSelector),
        }
    }

    /// Crop the cell at `(row, col)` and encode it as a PNG sprite.
    pub fn cut(&self, img: &DynamicImage, row: u32, col: u32) -> Result<Sprite, SpriteError> {
        let (x, y) = self.origin(row, col);

        let cell = img.crop_imm(x, y, self.tile_w, self.tile_h);

        let mut png = Vec::new();
        cell.write_to(&mut Cursor::new(&mut png), ImageFormat::Png)?;

        Ok(Sprite {
            row,
            col,
            x,
            y,
            width: self.tile_w,
            height: self.tile_h,
            png,
        })
    }

    /// Verify the resolved grid stays within the sheet bounds.
    fn ensure_fits(&self, width: u32, height: u32) -> Result<(), SpriteError> {
        let span = |margin: u32, tiles: u32, tile: u32, spacing: u32| {
            // margin*2 + tiles*tile + (tiles - 1)*spacing, saturating so an
            // overflow simply reports as "does not fit".
            margin
                .saturating_mul(2)
                .saturating_add(tiles.saturating_mul(tile))
                .saturating_add(tiles.saturating_sub(1).saturating_mul(spacing))
        };

        let used_w = span(self.margin_x, self.cols, self.tile_w, self.spacing_x);
        let used_h = span(self.margin_y, self.rows, self.tile_h, self.spacing_y);

        if used_w > width || used_h > height {
            return Err(SpriteError::DoesNotFit {
                width,
                height,
                rows: self.rows,
                cols: self.cols,
                margin_x: self.margin_x,
                margin_y: self.margin_y,
                spacing_x: self.spacing_x,
                spacing_y: self.spacing_y,
            });
        }

        Ok(())
    }
}

/// Grid mode: derive the tile size along one axis, requiring an even division.
fn tile_size(
    axis: &'static str,
    length: u32,
    count: u32,
    margin: u32,
    spacing: u32,
) -> Result<u32, SpriteError> {
    if count == 0 {
        return Err(SpriteError::ZeroDimension);
    }

    let usable = length
        .checked_sub(margin.saturating_mul(2))
        .and_then(|len| len.checked_sub(count.saturating_sub(1).saturating_mul(spacing)))
        .ok_or(SpriteError::NonDivisibleGrid {
            axis,
            usable: 0,
            count,
            leftover: 0,
        })?;

    let leftover = usable % count;

    if leftover != 0 {
        return Err(SpriteError::NonDivisibleGrid {
            axis,
            usable,
            count,
            leftover,
        });
    }

    Ok(usable / count)
}

/// Size mode: how many whole tiles of `tile` pixels fit along one axis.
fn tile_fit(length: u32, tile: u32, margin: u32, spacing: u32) -> Result<u32, SpriteError> {
    if tile == 0 {
        return Err(SpriteError::ZeroDimension);
    }

    let usable = length.saturating_sub(margin.saturating_mul(2));

    // n*tile + (n-1)*spacing <= usable  =>  n <= (usable + spacing) / (tile + spacing)
    let count = usable.saturating_add(spacing) / (tile + spacing);

    Ok(count)
}
