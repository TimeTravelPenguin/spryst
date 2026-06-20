// This file produces a demo "sprite sheet" of alphanumeric characters. Change `cols` to
// any number to get a different layout. When compiling, ensure you explicitly set the
// `--ppi` flag to match the `ppi` variable in this file, so that the cells are sized
// correctly to 32 pixels each. Compile with:
//
//     typst compile --ppi 300 sprite.typ sprite.png
//
// Requires the font [Buenard](https://fonts.google.com/specimen/Buenard).

#let ALPHA = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
#let NUMERIC = "0123456789"

#let make-chars(cols, sep: false) = {
  let chars = if sep {
    let rem = calc.rem(ALPHA.len(), cols)
    ALPHA + " " * (cols - rem) + NUMERIC
  } else {
    ALPHA + NUMERIC
  }

  chars.clusters()
}

#let make-sheet(cols, ppi, sep: false, cell-px: 32) = {
  let chars = make-chars(cols, sep: sep)
  let cell = cell-px / ppi * 72pt
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

  pagebreak(weak: true)
}
