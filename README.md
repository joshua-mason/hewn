# Hewn

Hewn is a primitive Rust game engine for learning and tinkering, with Terminal and WebAssembly support.

- Crate: [crates.io/hewn](https://crates.io/crates/hewn)
- Examples: `examples/asciijump`, `examples/asciibird`, `examples/snake`

Quick start:

```bash
# Terminal
cargo run -p asciijump

# Web (serve locally)
# Install wasm-pack if you haven't already
# https://drager.github.io/wasm-pack/installer/
cd examples/asciijump
wasm-pack build --release --target web
python3 -m http.server
```
