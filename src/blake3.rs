use std::io::Read;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Blake3;

#[wasm_bindgen]
impl Blake3 {
    pub fn blake3_xof(data: &[u8], length: usize) -> Vec<u8> {
        let mut buf = vec![0_u8; length];
        let mut hasher = blake3::Hasher::new();
        hasher.update(data);
        hasher.finalize_xof().read_exact(&mut buf).unwrap();
        buf
    }
}