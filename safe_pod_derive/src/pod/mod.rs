// Define modules
mod attributes;
mod r#struct;
mod r#enum;

use proc_macro2::TokenStream;
use syn::{Data, DeriveInput, Error};

/// Implementation of derive macro for Pod trait
pub fn derive_pod_impl(input: DeriveInput) -> TokenStream {
    // If the type that derives Pod is a union
    // return error
    if let Data::Union(d) = &input.data {
        return Error::new(
            d.union_token.span,
            "Union types cannot derive Pod"
        ).to_compile_error();
    }

    // If the type that derives Pod is a struct
    if let Data::Struct(d) = &input.data {
        let name = input.ident;
        return r#struct::derive_struct_impl(name, d);
    }

    // If the type that derives Pod is an enum
    if let Data::Enum(d) = &input.data {
        let name = input.ident;
        return r#enum::derive_enum_impl(name, input.attrs, d);
    }

    return TokenStream::new();
}
