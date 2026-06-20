WASM_OUT := "./typst/wasm/spryst.wasm"

install:
    rustup target add wasm32-unknown-unknown
    rustup target add wasm32-wasip1
    cargo binstall wasi-stub

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
