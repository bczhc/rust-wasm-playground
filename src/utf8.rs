use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Utf8;

#[wasm_bindgen]
impl Utf8 {
    #[allow(clippy::wrong_self_convention)]
    pub fn to_utf8(s: &str) -> Vec<u8> {
        s.as_bytes().into()
    }
}