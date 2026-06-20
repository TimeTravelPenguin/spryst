#import "util.typ": make-sheet

#let ppi = 300

#let cols = (7, 8, 9)

#for col in cols {
  for sep in (false, true) {
    make-sheet(col, ppi, sep: sep)
  }
}
