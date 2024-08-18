use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/static/script.js")]
extern "C" {
    pub fn downloadFile(filename: &str, data: js_sys::Uint8Array);
}