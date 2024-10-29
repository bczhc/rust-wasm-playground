use crate::errors::{AnyhowExt, ResultExt};
use crate::hashes::{ripemd160, hash160, sha256, sha256d, DigestType, sha1};
use bitcoin::ScriptBuf;
use std::str::FromStr;
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

    pub fn base58_check(hex: &str) -> crate::Result<String> {
        let result: anyhow::Result<_> = try { bitcoin::base58::encode_check(&hex::decode(hex)?) };
        result.map_err_string()
    }

    pub fn parse_hex_str(hex: &str) -> crate::Result<Vec<u8>> {
        let result = hex::decode(hex);
        result.map_err_string()
    }

    pub fn digest(data: &[u8], name: &str) -> crate::Result<String> {
        let result: anyhow::Result<_> = try {
            let r#type = DigestType::from_str(name)?;
            match r#type {
                DigestType::Ripemd160 => hex::encode(ripemd160(data)),
                DigestType::Sha256 => hex::encode(sha256(data)),
                DigestType::Sha256d => hex::encode(sha256d(data)),
                DigestType::Hash160 => hex::encode(hash160(data)),
                DigestType::Sha1 => hex::encode(sha1(data)),
            }
        };
        result.map_err_string()
    }
}
