# spryst

A [Typst](https://typst.app) WASM plugin for slicing a spritesheet into its
individual sprites — give it an image and a grid, get back ready-to-place
sprites.

## What it does

`spryst` takes the raw bytes of a spritesheet (PNG, JPEG, GIF, or WebP) and cuts
it into a grid of sprites, each returned as a PNG. You describe the grid in one
of two ways and the plugin works out the rest, honouring any border (`margin`)
and inter-tile gap (`spacing`).

- **Grid mode** — give `rows` and `cols`; the tile size is derived and must
  divide the usable area evenly.
- **Pixel mode** — give `tile-width` and `tile-height`; the row/column counts are
  derived as the number of whole tiles that fit.

## Usage

The easiest entry point is the `spryst.typ` wrapper:

```typ
#import "spryst.typ"

#let data = read("spritesheet.png", encoding: none)

// Inspect the sheet without decoding any sprites.
#let nfo = spryst.sheet-info(data, rows: 4, cols: 4)
// => (sheet_width, sheet_height, rows, cols, tile_width, tile_height, count)

// Pull out a single sprite, by index (row-major) or by (row, col).
#spryst.sprite-image(spryst.sprite(data, index: 5, rows: 4, cols: 4), width: 32pt)
#spryst.sprite-image(spryst.sprite(data, row: 1, col: 1, rows: 4, cols: 4))

// Or slice the whole sheet and lay every sprite out.
#let sheet = spryst.split(data, rows: 4, cols: 4)
#grid(
  columns: sheet.cols,
  ..sheet.sprites.map(spr => spryst.sprite-image(spr, width: 24pt)),
)
```

### Margin and spacing

Both are optional and default to `0`. Pass a single number to apply it to both
axes, or a `(x, y)` array for per-axis control. A `margin` is the border between
the sheet edge and the outermost tiles; `spacing` is the gap between adjacent
tiles.

```typ
#spryst.split(data, rows: 4, cols: 4, margin: 1, spacing: (2, 2))
```

### Pixel mode

```typ
#spryst.split(data, tile-width: 16, tile-height: 16)
```

## Plugin functions (low level)

Each function takes the sheet bytes plus CBOR-encoded arguments and returns a
CBOR-encoded response. Errors are returned as `Err` and surfaced by Typst as
diagnostics.

| Function | Arguments | Returns |
| --- | --- | --- |
| `split(sheet, spec)` | sheet bytes, CBOR `SliceSpec` | `{ rows, cols, tile_width, tile_height, sprites: [...] }` |
| `sprite(sheet, spec, selector)` | sheet bytes, CBOR `SliceSpec`, CBOR `Selector` | one sprite dict |
| `info(sheet, spec)` | sheet bytes, CBOR `SliceSpec` | `{ sheet_width, sheet_height, rows, cols, tile_width, tile_height, count }` |

A sprite dict is `{ row, col, x, y, width, height, png }`, where `png` is a CBOR
byte string (decoded directly to Typst `bytes`).

`SliceSpec` fields: `rows`, `cols`, `tile_width`, `tile_height` (provide one
pair), plus `margin_x`, `margin_y`, `spacing_x`, `spacing_y` (default `0`).
`Selector` fields: `index`, or both `row` and `col`.

## Building

```sh
just install   # one-time: wasm targets + wasi-stub
just build     # builds build/spryst.wasm
```

The Rust logic is unit-tested on the host:

```sh
cargo test
```

The plugin is also tested end-to-end through Typst with
[tytanic](https://typst-community.github.io/tytanic/). Each test renders an
alphanumeric spritesheet, slices it back apart with `spryst`, re-lays the
sprites into a grid, and checks the result is pixel-identical to the original —
so a slicing or coordinate-maths regression fails the build.

```sh
just test-typst                       # run every case (regenerates fixtures if missing)
just test-typst reassemble/c8-plain   # one test
just test-typst -e 'glob:"*sep*"'     # a test-set expression
```

The PNG fixtures under `typst/tests/fixtures/` are committed. Regenerate them
with `just gen-fixtures` after changing the cases, the glyph set, or the PPI
(`PPI` in `typst/tests/lib.typ`, mirrored by `default.ppi` in `typst.toml`).
Both require the [Buenard](https://fonts.google.com/specimen/Buenard) font.

> **Note:** `build/` is git-ignored. If you distribute this as a Typst package,
> commit the built `build/spryst.wasm` (or adjust the `plugin(...)` path in
> `spryst.typ`) so consumers don't have to compile it themselves.
