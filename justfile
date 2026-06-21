WASM_OUT := "./typst/wasm/spryst.wasm"

install:
    rustup target add wasm32-unknown-unknown
    rustup target add wasm32-wasip1
    cargo binstall wasi-stub

assets:
    #!/usr/bin/env bash
    set -euo pipefail

    cd typst/assets 
    
    typst c --root .. --ppi 300 sheet.typ sheet.png
    typst c --root .. --ppi 300 banner.typ banner.png
    typst c --root .. --ppi 300 --input "banner=github" banner.typ banner_1280_640.png
    
    oxipng *.png

build:
    cargo build \
      --release \
      --target wasm32-wasip1 \
      --target-dir rust/target \
      --manifest-path rust/Cargo.toml

    mkdir -p typst/wasm
    cp rust/target/wasm32-wasip1/release/spryst.wasm {{ WASM_OUT }}
    wasi-stub {{ WASM_OUT }} -o {{ WASM_OUT }}

clean:
    rm -rf typst/wasm
    rm -rf rust/target

# Render the spritesheet PNG fixtures the Typst tests slice (commit the result).
# Re-run after changing the cases, the glyph set, or PPI.
gen-fixtures:
    #!/usr/bin/env bash
    set -euo pipefail

    ppi=300
    fixtures="typst/tests/fixtures"

    # tag => "cols sep"; tags match the read(...) paths in each test's typ files.
    cases=(
      "c7-plain:7 false"
      "c7-sep:7 true"
      "c8-plain:8 false"
      "c8-sep:8 true"
      "c9-plain:9 false"
      "c9-sep:9 true"
    )

    # Capture first; piping into `grep -q` would SIGPIPE typst and trip pipefail.
    if ! typst fonts | grep -i buenard >/dev/null; then
      echo "error: the Buenard font is not installed (https://fonts.google.com/specimen/Buenard)" >&2
      exit 1
    fi

    mkdir -p "$fixtures"

    for case in "${cases[@]}"; do
      tag="${case%%:*}"
      read -r cols sep <<<"${case#*:}"

      typst compile --root typst --ppi "$ppi" \
        --input "cols=$cols" --input "ppi=$ppi" --input "sep=$sep" \
        typst/tests/source.typ "$fixtures/$tag.png"

      echo "wrote fixtures/$tag.png (cols=$cols sep=$sep)"
    done

# Run the Typst round-trip tests with tytanic (extra args go to `tt run`).
# Generates fixtures first if missing. E.g. `just test-typst -e 'glob:"*sep*"'`.
test-typst *args:
    #!/usr/bin/env bash
    set -euo pipefail

    if ! compgen -G "typst/tests/fixtures/*.png" >/dev/null; then
      just gen-fixtures
    fi

    # --no-export-ephemeral: our tests are ephemeral, so skip writing the
    # regenerated reference PNGs into each test's ref/ dir on every run.
    tt run --root typst --no-export-ephemeral {{ args }}
