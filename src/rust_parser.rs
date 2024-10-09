use crate::errors::AnyhowExt;
use quote::quote;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct RustParser;

#[wasm_bindgen]
pub struct ParseResult {
    syntax: String,
    reassembled: String,
}

#[wasm_bindgen]
impl ParseResult {
    #[wasm_bindgen(getter)]
    pub fn syntax(&self) -> String {
        self.syntax.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn reassembled(&self) -> String {
        self.reassembled.clone()
    }
}

#[wasm_bindgen]
impl RustParser {
    pub fn parse_file(code: &str) -> Result<ParseResult, String> {
        let result: anyhow::Result<_> = try {
            let syntax = syn::parse_file(code)?;
            let syntax_str = format!("{:#?}", syntax);
            let reassembled = quote!(#syntax);
            ParseResult {
                syntax: syntax_str,
                reassembled: format!("{reassembled}"),
            }
        };
        result.map_err_string()
    }
}
