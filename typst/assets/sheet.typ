#let CELL_PX = 128
#let cols = 9
#let ppi = 300
#let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".clusters()
#let cell = CELL_PX / ppi * 72pt
#let rows = calc.ceil(chars.len() / cols)

#let typ-text = text.with(
  font: "Buenard",
  weight: "bold",
  size: cell * 0.85,
  top-edge: "bounds",
  bottom-edge: "bounds",
  fill: rgb("#239dad"),
)

#set page(
  fill: none,
  width: cols * cell,
  height: rows * cell,
  margin: 0pt,
)

#grid(
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


