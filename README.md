## Install deps
```
cargo install wasm-bindgen-cli https
```

## Build
```
cargo build --target wasm32-unknown-unknown
```

## Generate WASM bindings
```
wasm-bindgen target/wasm32-unknown-unknown/debug/wasm_slicer.wasm --out-dir . --target web --no-typescript
```

## Serve
```
http
```