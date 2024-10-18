use crate::errors::AnyhowExt;
use bitcoin::ScriptBuf;
use digest::Update;
use ripemd::Ripemd160;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Bitcoin;

#[wasm_bindgen]
impl Bitcoin {
    pub fn parse_script_hex(hex_string: &str) -> crate::Result<String> {
        let result: anyhow::Result<_> = try {
            let vec = hex::decode(hex_string)?;
            let script_buf = ScriptBuf::from(vec);
            script_buf.to_asm_string()
        };
        result.map_err_string()
    }

    pub fn ripemd160(hex: &str) -> crate::Result<String> {
        let result: anyhow::Result<_> = try {
            let mut hasher = Ripemd160::default();
            hasher.update(&hex::decode(hex)?);
            let output = digest::FixedOutput::finalize_fixed(hasher);
            hex::encode(output.as_slice())
        };
        result.map_err_string()
    }

    pub fn base58_check(hex: &str) -> crate::Result<String> {
        let result: anyhow::Result<_> = try {
            bitcoin::base58::encode_check(&hex::decode(hex)?)
        };
        result.map_err_string()
    }
}