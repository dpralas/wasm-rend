# wgpu based STL rendering engine
## Inspiration:

* https://github.com/gfx-rs/wgpu
* https://github.com/simbleau/nbody-wasm-sim
* https://github.com/iced-rs/iced/tree/master/examples/integration_wgpu

## Steps:

### 0. Install Rust and add WASM target
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown
```

### 1. Install helper tools
```
cargo install wasm-bindgen-cli https
```

### 2. Build
```
cargo build --target wasm32-unknown-unknown
```

### 3. Generate WASM bindings
```
wasm-bindgen target/wasm32-unknown-unknown/debug/wasm_slicer.wasm --out-dir . --target web --no-typescript
```

### 4. Serve
```
http
```