build_wasm:
	wasm-pack build --target web
build_cargo:
	cargo build --target wasm32-unknown-unknown
rust2html:
	rustc --target wasm32-unknown-emscripten hello.rs -o hello.html
