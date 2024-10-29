#![feature(try_blocks)]
#![feature(isqrt)]

use std::fmt::Debug;
use std::str::FromStr;

use wasm_bindgen::convert::ReturnWasmAbi;
use wasm_bindgen::prelude::*;
use wasm_bindgen::prelude::*;

mod utils;
mod argon2;
mod errors;
mod exif;
mod ipv6;
mod blake3;
mod hex;
mod utf8;
mod rust_parser;
mod leg_cpu;
mod kaprekar;
mod bitcoin;
mod hashes;
mod log;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
fn wasm_init() {
    utils::set_panic_hook();
}

pub type Result<T> = std::result::Result<T, String>;
