# wgpu based STL rendering engine
### Inspirations:
    * https://github.com/gfx-rs/wgpu
    * https://github.com/simbleau/nbody-wasm-sim

## 1. Install deps
```
cargo install wasm-bindgen-cli https
```

## 2. Build
```
cargo build --target wasm32-unknown-unknown
```

## 3. Generate WASM bindings
```
wasm-bindgen target/wasm32-unknown-unknown/debug/wasm_slicer.wasm --out-dir . --target web --no-typescript
```

## 4. Serve
```
http
```