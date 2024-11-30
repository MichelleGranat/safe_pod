mod zeroable;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro defenition
#[proc_macro_derive(Zeroable)]
pub fn derive_zeroable(input: TokenStream) -> TokenStream {
    zeroable::derive_zeroable_impl(
        parse_macro_input!(input as DeriveInput)
    ).into()
}