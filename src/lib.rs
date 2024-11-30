//! # Safe Pod
//! ``safe_pod`` hepls creating types that can be serialized
//! to a byte array (``&[u8]``) and deserialyzed from one, all
//! written in safe Rust.
//! 
//! It can particularly useful when parsing binary file formats.
//! 
//! # Getting started
//! To get started add ``safe_pod`` to your project by running:
//! ```
//! cargo add safe_pod
//! ```
//! or by adding the following to your ``Cargo.toml`` file:
//! ```
//! safe_pod = "0.0.1"
//! ```
//! 
//! # Basic use
//! The following primitive types implement ``Zeroable`` and ``Pod`` traits:
//! ``bool``, ``u8``, ``u16``, ``u32``, ``u64``, ``u128``, ``i8``, 
//! ``i16``, ``i32``, ``i64``, ``128``, ``f32``, ``f64``.
//! 
//! Any struct where all fields are of types that implement 
//! ``Zeroable`` or ``Pod`` can derive those traits respectively.
//! 
//! ```
//! #[derive(Debug, Zeroable, Pod)]
//! struct Foo {
//!     a: i8,
//!     b: bool,
//!     c: f32,
//! }
//! 
//! let zeroed_foo = Foo::zeroed();
//! let foo_from_bytes = Foo::from_le_bytes(&[0, 1, 0, 0, 0, 0])?;
//! let mut bytes_from_foo = [0u8; Foo::SIZE];
//! let bytes_written = foo_from_bytes.to_be_bytes(&mut bytes_from_foo)?;
//! 
//! println!("Foo zeroed: {:#?}", zeroed_foo);
//! println!("Foo from bytes: {:#?}", foo_from_bytes);
//! println!("Foo wrote {} bytes to byte buffer: {:#?}", bytes_written, bytes_from_foo);
//! ```
//! 
//! # Upcoming
//! In following versions more primitive types will be supported,
//! some ``std`` types will be supported, and the derive macros will
//! be more flexible.

// Define modules
mod zeroable;
mod pod;

// Re-exports
pub use zeroable::Zeroable;
pub use pod::{Pod, PodError};
pub use safe_pod_derive::{
    Zeroable,
    Pod
};
