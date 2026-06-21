// spryst reassemble (size mode): slice the c7-sep fixture and re-grid the
// sprites — must reproduce the fixture pixel-for-pixel.
#import "/tests/lib.typ": rebuilt

#rebuilt(read("/tests/fixtures/c7-sep.png", encoding: none), 7, sep: true, mode: "size")
