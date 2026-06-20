// spryst reassemble (grid mode): slice the c8-sep fixture and re-grid the
// sprites — must reproduce the fixture pixel-for-pixel.
#import "/tests/lib.typ": rebuilt

#rebuilt(read("/tests/fixtures/c8-sep.png", encoding: none), 8, sep: true, mode: "grid")
