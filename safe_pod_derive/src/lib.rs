mod zeroable;
mod pod;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Zeroable derive macro defenition
#[proc_macro_derive(Zeroable)]
pub fn derive_zeroable(input: TokenStream) -> TokenStream {
    zeroable::derive_zeroable_impl(
        parse_macro_input!(input as DeriveInput)
    ).into()
}

/// Pod derive macro defenition
#[proc_macro_derive(Pod)]
pub fn derive_pod(input: TokenStream) -> TokenStream {
    pod::derive_pod_impl(
        parse_macro_input!(input as DeriveInput)
    ).into()
}