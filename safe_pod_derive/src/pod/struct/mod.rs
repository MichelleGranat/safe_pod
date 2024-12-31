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

// / A representation of a struct 
// / that derives Zeroable trait
// pub struct DeriveZeroableStruct {
//     name: Ident,
//     fields: Vec<(Ident, Type)>,
// }

// impl DeriveZeroableStruct {
//     /// Returns the Zeroable implementation for the struct
//     pub fn implementation(self) -> TokenStream {
//         // Generate zeroed fields instructions
//         let mut zeroed_fields: Vec<TokenStream> = Vec::new();

//         for (n, t) in self.fields {
//             let ty_span = t.span();
//             // Add zeroing
//             zeroed_fields.push(
//                 quote_spanned!(ty_span => #n: <#t as safe_pod::Zeroable>::zeroed())
//             );
//         }

//         let name = &self.name;

//         quote! {
//             impl safe_pod::Zeroable for #name {
//                 #[inline]
//                 fn zeroed() -> Self {
//                     Self { #(#zeroed_fields),* }
//                 }
//             }
//         }.to_token_stream()
//     }
// }