// spryst reassemble (pixel mode): slice the c7-plain fixture and re-grid the
// sprites — must reproduce the fixture pixel-for-pixel.
#import "/tests/lib.typ": rebuilt

#rebuilt(read("/tests/fixtures/c7-plain.png", encoding: none), 7, sep: false, mode: "pixel")
