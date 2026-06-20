// spryst reassemble (pixel mode): slice the c9-sep fixture and re-grid the
// sprites — must reproduce the fixture pixel-for-pixel.
#import "/tests/lib.typ": rebuilt

#rebuilt(read("/tests/fixtures/c9-sep.png", encoding: none), 9, sep: true, mode: "pixel")
