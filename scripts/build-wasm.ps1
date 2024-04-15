cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name "ld55" ./target/wasm32-unknown-unknown/release/ld55.wasm
wasm-opt -Oz -o ./out/ld55_bg.wasm ./out/ld55_bg.wasm