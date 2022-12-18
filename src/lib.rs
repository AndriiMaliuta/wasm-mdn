use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[no_mangle]
fn add(x: i32, y:i32) -> i32 {x + y}

#[wasm_bindgen]
pub fn parse_text(text: String) -> String {
    return text.replace("lorem", "LOREM");
}