#import "@preview/catppuccin:1.1.0": catppuccin
#import "spryst.typ": *

#show: catppuccin.with("mocha")

#let img = read("32x32_PixelWeapons_Free.png", encoding: none)

#let info = sheet-info(img, tile-width: 32, tile-height: 32)
#repr(info)

#let sheet = split(img, tile-width: 32, tile-height: 32)
#grid(
  columns: sheet.cols,
  // ..sheet.sprites.map(sprite-image),
  ..sheet.sprites.map(sh => image(sh.png))
)
