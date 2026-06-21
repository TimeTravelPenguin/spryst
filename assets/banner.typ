#let (width, height, margin, text-sz) = if (
  sys.inputs.at("banner", default: none) == "github"
) {
  (1280in / 300, 640in / 300, 0em, 0.9em)
} else {
  (2560in / 300, auto, 1.5em, 1.2em)
}

#set page(
  width: width,
  height: height,
  margin: margin,
  fill: white,
)

#let CELL_SZ = 1.5em
#set text(size: text-sz)

#let typ-text-size = CELL_SZ * 0.85
#let typ-text = text.with(
  font: "Buenard",
  weight: "bold",
  size: typ-text-size,
  top-edge: "bounds",
  bottom-edge: "bounds",
  fill: rgb("#239dad"),
)

#let alphanum = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
#let chars = (
  alphanum
    .clusters()
    .map(typ-text)
    .enumerate()
    .map(((idx, c)) => if idx == 18 {
      box(width: CELL_SZ, height: CELL_SZ, align(center + horizon, [
        #place(top + right, dx: -0.1em, dy: 0.1em, text(
          size: 0.3em,
          fill: rgb("#C92B89"),
          `x2`,
        ))
        #c
      ]))
    } else {
      box(width: CELL_SZ, height: CELL_SZ, align(center + horizon, c))
    })
)


#let cells = block(
  // stroke: 0.05em + gray,
  {
    let selected = "SPRYST".clusters().map(c => alphanum.position(c))

    grid(
      columns: 9,
      column-gutter: 0em,
      stroke: (x, y) => {
        let idx = x + 9 * y
        if idx in selected {
          (thickness: 0.05em, paint: rgb("#C92B89"), dash: "densely-dashed")
        } else {
          none
        }
      },
      ..chars,
    )
  },
)

#let spryst = grid(
  columns: 6,
  .."SPRYST"
    .clusters()
    .map(typ-text.with(size: typ-text-size * 1.5))
    .map(c => box(
      stroke: (
        thickness: 0.04em,
        paint: gray,
        dash: "densely-dashed",
      ),
      width: CELL_SZ * 1.5,
      height: CELL_SZ * 1.5,
      align(
        center + horizon,
        c,
      ),
    ))
)

#show "spritesheet": set text(fill: rgb("#005ECB"))
#show "sheet-png": set text(fill: rgb("#C92B89"))
#show "let": set text(fill: rgb("#F2001F"))
#show regex("[()]"): set text(fill: rgb("#0082DF"))
#show "=": set text(fill: rgb("#E24C00"))
#show "#": set text(fill: rgb("#8861F3"))

#let let_text = stack(dir: ltr, ```typ #let sheet-png = ```, scale(
  cells,
  75%,
  reflow: true,
))

#let call_text = ```typ #spritesheet(sheet-png)```

#set align(center + horizon)
#stack(
  dir: ttb,
  spacing: 1em,
  let_text,
  stack(
    dir: ltr,
    spacing: 0.3em,
    call_text,
    text(size: 1.5em, fill: luma(160))[#sym.arrow.r],
    spryst,
  ),
)

