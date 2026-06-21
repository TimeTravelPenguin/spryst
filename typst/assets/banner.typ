#import "../src/lib.typ": split
#import "sheet.typ": CELL_PX

#let (width, height, margin) = if (
  sys.inputs.at("banner", default: none) == "github"
) {
  (1280in / 300, 640in / 300, 0em)
} else {
  (2560in / 300, auto, 2em)
}

#set page(
  width: width,
  height: height,
  margin: margin,
  fill: white,
)

#set text(size: 2em)

#let sheet = read("sheet.png", encoding: none)
#let sprites = split(sheet, tile-width: CELL_PX, tile-height: CELL_PX).sprites

#let sprite_idxs = (18, 15, 17, 24, 18, 19)

#let sheet = {
  let selected = sprite_idxs
    .map(idx => sprites.at(idx))
    .map(spr => (spr.col, spr.row))

  grid(
    columns: 9,
    column-gutter: 0em,
    stroke: (x, y) => if (x, y) in selected {
      (thickness: 0.05em, paint: rgb("#C92B89"))
    } else {
      none
    },
    ..sprites.map(spr => image(spr.png, height: 0.5em)),
  )
}

#let spryst = grid(
  columns: 7,
  column-gutter: 0em,
  ..sprite_idxs.map(idx => image(sprites.at(idx).png, height: 1.5em)),
)


#let split_text = text(
  "split",
  font: "Fira Code",
  size: 1.5em,
  fill: rgb("#005ECB"),
)

#let parens = {
  set text(fill: rgb("#0082DF"), size: 1.5em)
  $ (#sheet) $
}

#set align(center + horizon)
#stack(
  dir: ltr,
  split_text,
  parens,
  text(size: 1.5em, fill: luma(160))[#sym.arrow.r],
  spryst,
)


