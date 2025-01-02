// Define modules
mod unit;
mod tuple;
mod named;

use proc_macro2::TokenStream;
use syn::{DataStruct, Fields, Ident};

/// Implementation of derive macro for structs
// TODO: add generics and attributes to function input
pub fn derive_struct_impl(name: Ident, struct_data: &DataStruct) -> TokenStream {
    // Match struct type and delegate to appropriate impl functions
    match &struct_data.fields {
        Fields::Unit => {
            return unit::derive_unit_struct_impl(&name);
        },
        Fields::Unnamed(f) => {
            return tuple::derive_tuple_struct_impl(&name, f);
        },
        Fields::Named(f) => {
            return named::derive_named_struct_impl(&name, f);
        }
    }
}
