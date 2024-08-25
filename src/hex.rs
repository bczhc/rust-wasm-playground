use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Hex;

#[wasm_bindgen]
impl Hex {
    pub fn encode(bytes: &[u8]) -> String {
        hex::encode(bytes)
    }

    pub fn decode(hex: &str) -> Option<Vec<u8>> {
        hex::decode(hex).ok()
    }
}