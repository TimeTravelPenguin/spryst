// Shared helpers for the spryst Typst tests.
//
// Two jobs live here:
//
//   * `make-sheet` renders an alphanumeric "spritesheet" with native Typst.
//     `source.typ` drives it from `sys.inputs` to generate the PNG fixtures
//     under `fixtures/` (see `gen-fixtures.sh`).
//
//   * `reference` and `rebuilt` are the two sides of each reassemble test.
//     `reference` places a fixture verbatim; `rebuilt` slices the same fixture
//     with the spryst plugin and re-lays the sprites into a grid. Both render a
//     page sized to exactly the fixture's pixels, so a correct slicer produces
//     a pixel-identical result and any coordinate drift shows up as a diff.
//
// Tests compile at `PPI` (mirrored in `typst.toml`); at that resolution a
// `CELL_PX`-pixel cell measures `CELL_PX / PPI * 72pt`, landing on whole pixels.

#import "/src/lib.typ": split

#let ALPHA = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
#let NUMERIC = "0123456789"

/// Pixels per inch the tests render at. Must match `default.ppi` in
/// `typst.toml` so cells map to whole pixels.
#let PPI = 300

/// Side length of a single sprite cell, in pixels.
#let CELL_PX = 32

/// The cell length in `pt` at the test resolution.
#let cell-len = CELL_PX / PPI * 72pt

/// The glyphs laid into the sheet. With `sep`, the alphabet is padded with
/// spaces so the digits begin on a fresh row.
///
/// - cols (int): number of columns
/// - sep (bool): pad the alphabet to a row boundary before the digits
///
/// -> array
#let make-chars(cols, sep: false) = {
  let chars = if sep {
    let rem = calc.rem(ALPHA.len(), cols)
    ALPHA + " " * (cols - rem) + NUMERIC
  } else {
    ALPHA + NUMERIC
  }

  chars.clusters()
}

/// Row count for a sheet of `cols` columns — `ceil(chars / cols)`. The fixture
/// is then `cols * CELL_PX` wide by `rows * CELL_PX` tall.
///
/// - cols (int): number of columns
/// - sep (bool): whether the digits are pushed onto a fresh row
///
/// -> int
#let row-count(cols, sep: false) = calc.ceil(make-chars(cols, sep: sep).len() / cols)

/// Renders the reference spritesheet natively — the glyphs in a gapless grid,
/// on a page sized to the grid exactly. Used to generate the PNG fixtures.
///
/// - cols (int): number of columns
/// - ppi (int): resolution the sheet will be rasterised at
/// - sep (bool): pad the alphabet to a row boundary before the digits
///
/// -> content
#let make-sheet(cols, ppi, sep: false) = {
  let chars = make-chars(cols, sep: sep)
  let cell = CELL_PX / ppi * 72pt
  let rows = calc.ceil(chars.len() / cols)

  let typ-text = text.with(
    font: "Buenard",
    weight: "bold",
    size: cell * 0.85,
    top-edge: "bounds",
    bottom-edge: "bounds",
  )

  set page(
    fill: none,
    width: cols * cell,
    height: rows * cell,
    margin: 0pt,
  )

  grid(
    columns: (cell,) * cols,
    rows: (cell,) * rows,
    column-gutter: 0pt,
    row-gutter: 0pt,

    ..chars.map(c => box(
      width: cell,
      height: cell,
      inset: 0pt,
      align(center + horizon, typ-text(c)),
    )),
  )
}

/// Places a fixture verbatim on a page sized to its pixels — the reference side of
/// a reassemble test, free of any spryst involvement.
///
/// - data (bytes): the fixture PNG
/// - cols (int): the sheet's column count
/// - sep (bool): whether the fixture was rendered in `sep` mode (sets row count)
///
/// -> content
#let reference(data, cols, sep: false) = {
  let rows = row-count(cols, sep: sep)

  set page(
    fill: none,
    width: cols * cell-len,
    height: rows * cell-len,
    margin: 0pt,
  )

  image(data, width: cols * cell-len, height: rows * cell-len)
}

/// Slices `data` into sprites with the spryst plugin and re-lays them into a
/// grid — the spryst side of a reassemble test. `mode` selects how the grid is
/// described to the plugin: `"pixel"` passes the tile size, `"grid"` passes the
/// row/column counts.
///
/// - data (bytes): the fixture PNG
/// - cols (int): the sheet's column count (only used in `"grid"` mode)
/// - sep (bool): whether the fixture was rendered in `sep` mode (only `"grid"`)
/// - mode (str): `"pixel"` or `"grid"`
///
/// -> content
#let rebuilt(data, cols, sep: false, mode: "pixel") = {
  let sheet = if mode == "grid" {
    split(data, rows: row-count(cols, sep: sep), cols: cols)
  } else {
    split(data, tile-width: CELL_PX, tile-height: CELL_PX)
  }

  let rows = calc.ceil(sheet.sprites.len() / sheet.cols)

  set page(
    fill: none,
    width: sheet.cols * cell-len,
    height: rows * cell-len,
    margin: 0pt,
  )

  grid(
    columns: (cell-len,) * sheet.cols,
    rows: (cell-len,) * rows,
    column-gutter: 0pt,
    row-gutter: 0pt,

    ..sheet.sprites.map(spr => box(
      width: cell-len,
      height: cell-len,
      inset: 0pt,
      image(spr.png, format: "png", width: cell-len, height: cell-len),
    )),
  )
}
