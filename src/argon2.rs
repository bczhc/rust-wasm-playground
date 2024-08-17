use std::convert::TryFrom;
use std::str::FromStr;

use argon2::{Algorithm, Params, PasswordHasher, Version};
use argon2::password_hash::{Salt, SaltString};
use argon2::password_hash::rand_core::OsRng;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::errors::AnyhowExt;

#[wasm_bindgen]
pub struct Argon2;

#[wasm_bindgen(js_class = "Argon2")]
impl Argon2 {
    pub fn hash(
        method: &str,
        password: &[u8],
        salt: &str,
        m_cost: u32,
        t_cost: u32,
        p_cost: u32,
        output_len: usize,
    ) -> Result<String, String> {
        let result: anyhow::Result<_> = try {
            let algorithm = Algorithm::from_str(method)?;

            let params = Params::new(m_cost, t_cost, p_cost, Some(output_len))?;
            let argon2 = argon2::Argon2::new(algorithm, Version::V0x13, params);

            argon2.hash_password(password, Salt::try_from(salt)?)?
        };

        result.map(|x| format!("{x}")).map_err_string()
    }

    pub fn random_salt() -> String {
        let salt = SaltString::generate(&mut OsRng);
        salt.to_string()
    }

    pub fn string_to_utf8(s: String) -> Box<[u8]> {
        // `s` here has already been guaranteed to be valid UTF-8 encoded in Rust
        s.into_boxed_str().into()
    }
}