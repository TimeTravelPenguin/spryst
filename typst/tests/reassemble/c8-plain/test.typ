// spryst reassemble (size mode): slice the c8-plain fixture and re-grid the
// sprites — must reproduce the fixture pixel-for-pixel.
#import "/tests/lib.typ": rebuilt

#rebuilt(read("/tests/fixtures/c8-plain.png", encoding: none), 8, sep: false, mode: "size")
