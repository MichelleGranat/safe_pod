//! Derive macros for ``safe_pod`` crate.

// Declare modules
mod zeroable;
mod pod;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro for the Zeroable trait
/// 
/// # Example
/// ```
/// #[derive(Debug, Zeroable)]
/// struct Foo {
///     a: i8,
///     b: bool,
///     c: f32,
/// }
/// 
/// let zeroed_foo = Foo::zeroed();
/// 
/// println!("Foo zeroed: {:#?}", zeroed_foo);
/// ```
#[proc_macro_derive(Zeroable, attributes(zero))]
pub fn derive_zeroable(input: TokenStream) -> TokenStream {
    zeroable::derive_zeroable_impl(
        parse_macro_input!(input as DeriveInput)
    ).into()
}

/// Derive macro for the Pod trait
/// 
/// # Example
/// ```
/// #[derive(Debug, Pod)]
/// struct Foo {
///     a: i8,
///     b: bool,
///     c: f32,
/// }
/// 
/// let foo_from_bytes = Foo::from_le_bytes(&[0, 1, 0, 0, 0, 0])?;
/// let mut bytes_from_foo = [0u8; Foo::SIZE];
/// let bytes_written = foo_from_bytes.to_be_bytes(&mut bytes_from_foo)?;
/// 
/// println!("Foo from bytes: {:#?}", foo_from_bytes);
/// println!("Foo wrote {} bytes to byte buffer: {:#?}", bytes_written, bytes_from_foo);
/// ```
#[proc_macro_derive(Pod, attributes(pod))]
pub fn derive_pod(input: TokenStream) -> TokenStream {
    pod::derive_pod_impl(
        parse_macro_input!(input as DeriveInput)
    ).into()
}