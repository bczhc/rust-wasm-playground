extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn handle_errors(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = parse_macro_input!(input as ItemFn);
    let tokens = quote! {
        #(#attrs)* #vis #sig {
            let r: ::anyhow::Result<_> = try {
                #block
            };
            crate::errors::AnyhowExt::map_err_string(r)
        }
    };
    tokens.into()
}
