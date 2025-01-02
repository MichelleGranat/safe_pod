//! Derive macros for ``safe_pod`` crate.

// Declare modules
mod zeroable;
mod pod;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro for the Zeroable trait
/// 
/// # Example
/// In order to derive the `Zeroable` trait for a struct,
/// all of its' fields must also implement `Zeroable`.
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
/// 
/// <br/>
/// 
/// In order to derive the `Zeroable` trait for an enum,
/// all of its' variants fields must also implement `Zeroable`.
/// The "zero variant" must be marked with `#[zero]`.
/// ```
/// #[derive(Debug, Zeroable)]
/// enum MyEnum {
///     UnitVariant,
///     TupleVariant(u8, f32),
///     #[zero]
///     StructVariant {
///         x: u32,
///         y: u32,
///     },
/// }
/// 
/// let zeroed_my_enum = MyEnum::zeroed();
/// 
/// println!("MyEnum zeroed: {:#?}", zeroed_my_enum);
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
/// In order to derive the `Pod` trait for a struct,
/// all of its' fields must also implement `Pod`.
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
/// 
/// <br />
/// 
/// In order to derive the `Pod` trait for an enum,
/// it must be a [unit-like enum](https://doc.rust-lang.org/reference/items/enumerations.html#unit-only-enum).
/// The enum must have the `#[pod(...)]` atrribute with the inner attribute `repr($type)`
/// set to a type that implements `Pod`. Every variant must also have the `#[pod(...)]` 
/// atrribute with the inner attribute `match_expr($expression)` set to an expression of the
/// type set in `repr($tpye)`.
/// ```
/// #[derive(Debug, Pod)]
/// #[pod(repr(u32))]
/// enum UnitLikeEnum {
///     #[pod(match_expr(0))]
///     Foo,
///     #[pod(match_expr(1))]
///     Bar,
/// }
/// 
/// let enum_from_bytes = UnitLikeEnum::from_le_bytes(&[1, 0, 0, 0])?;
/// let mut bytes_from_enum = [0u8; UnitLikeEnum::SIZE];
/// let bytes_written = enum_from_bytes.to_be_bytes(&mut bytes_from_enum)?;
/// 
/// println!("UnitLikeEnum from bytes: {:#?}", enum_from_bytes);
/// println!("UnitLikeEnum wrote {} bytes to byte buffer: {:#?}", bytes_written, bytes_from_enum);
/// ```
#[proc_macro_derive(Pod, attributes(pod))]
pub fn derive_pod(input: TokenStream) -> TokenStream {
    pod::derive_pod_impl(
        parse_macro_input!(input as DeriveInput)
    ).into()
}