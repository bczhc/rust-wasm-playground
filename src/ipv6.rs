use std::convert::TryInto;
use std::net::Ipv6Addr;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Ipv6;

#[wasm_bindgen]
impl Ipv6 {
    pub fn parse_ipv6(s: &str) -> Option<Vec<u8>> {
        s.parse::<Ipv6Addr>().ok().map(|x| x.octets().into())
    }

    pub fn bytes_to_ipv6(bytes: &[u8]) -> Option<String> {
        let bytes: [u8; 16] = bytes.try_into().ok()?;
        Some(format!("{}", Ipv6Addr::from(bytes)))
    }
}