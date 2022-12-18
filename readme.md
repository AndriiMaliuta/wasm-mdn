

### wasm-bindgen
wasm-bindgen provides options to generate the binding file for both Node.js and the browser environment. Let's see those flags:

- --nodejs: This generates output that only works for Node.js. No ESModules.
- --browser: This generates output that only works for the browser. With ESModules.
- --no-modules: This generates output that only works for the browser. No ESModules. Suitable for browsers that don't support ESModules yet.

Generate the WebAssembly module using Cargo:
```bash
cargo build --target=wasm32-unknown-unknown
```

Use the wasm-bindgen CLI to generate the binding file for the WebAssembly module generated:
```
wasm-bindgen target/wasm32-unknown-unknown/debug/class_world.wasm --out-dir .
```

```
cargo expand --target=wasm32-unknown-unknown > expanded.rs
```

Note: Error message "error:0308010C:digital envelope routines::unsupported"
```
export NODE_OPTIONS=--openssl-legacy-provider
npm run serve
```