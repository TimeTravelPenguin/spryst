#import "@preview/spryst:0.1.0": make-getter, spritesheet

#set page(width: auto, height: auto)

#let data = read("../assets/sheet.png", encoding: none)
#let sheet = spritesheet(data, rows: 4, cols: 9)
#let get = make-getter(sheet)

#let indexes = (18, 15, 17, 24, 18, 19)
#stack(dir: ltr, ..indexes.map(i => get(i)))
