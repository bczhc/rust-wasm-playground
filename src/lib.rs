#![feature(try_blocks)]

use std::fmt::Debug;
use std::str::FromStr;

use wasm_bindgen::convert::ReturnWasmAbi;
use wasm_bindgen::prelude::*;
use wasm_bindgen::prelude::*;

mod utils;
mod argon2;
mod errors;

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
