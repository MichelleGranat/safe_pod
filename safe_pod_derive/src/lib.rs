use proc_macro::TokenStream;

#[proc_macro_derive(Zeroable)]
pub fn derive_zeroable(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}