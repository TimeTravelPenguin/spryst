// Fixture generator. Renders one reference spritesheet, parameterised entirely
// by `sys.inputs`, so `gen-fixtures.sh` can stamp out every case with plain
// `typst compile --input ...`. Tytanic itself never runs this file — it only
// consumes the PNGs it produces.
//
// Inputs (all strings, passed via `--input key=value`):
//   cols — number of columns in the grid
//   ppi  — pixels-per-inch the sheet will be rasterised at
//   sep  — "true" to pad the alphabet so digits start on a fresh row
//
// Compile at the matching `--ppi` so each cell is exactly 32px:
//
//     typst compile --root typst --ppi 144 \
//       --input cols=7 --input ppi=144 --input sep=false \
//       typst/tests/source.typ fixture.png

#import "/tests/lib.typ": make-sheet

#let cols = int(sys.inputs.cols)
#let ppi = int(sys.inputs.ppi)
#let sep = sys.inputs.sep == "true"

#make-sheet(cols, ppi, sep: sep)
