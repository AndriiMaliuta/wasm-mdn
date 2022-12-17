

### wasm-bindgen
wasm-bindgen provides options to generate the binding file for both Node.js and the browser environment. Let's see those flags:

- --nodejs: This generates output that only works for Node.js. No ESModules.
- --browser: This generates output that only works for the browser. With ESModules.
- --no-modules: This generates output that only works for the browser. No ESModules. Suitable for browsers that don't support ESModules yet.


```bash
cargo build --target=wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/wasm_mdn.wasm --out-dir .
cargo expand --target=wasm32-unknown-unknown > expanded.rs
```