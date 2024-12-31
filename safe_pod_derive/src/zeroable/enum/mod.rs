// Define modules
mod unit;
mod tuple;
mod r#struct;

use proc_macro2::TokenStream;
use syn::{DataEnum, Error, Fields, Ident};

/// Implementation of derive macro for enums
pub fn derive_enum_impl(name: Ident, enum_data: &DataEnum) -> TokenStream {
    // Find zero variant
    for variant in &enum_data.variants {
        // If it doesn't have attributes move to the next variant
        if variant.attrs.is_empty() {
            continue;
        }

        // Check if current variant is the zero variant
        let is_zero_variant = variant.attrs.iter().find(
            |attr| {
                attr.meta.path().is_ident("zero")
            }
        ).is_some();

        if is_zero_variant {
            // Match variant type and delegate to appropriate impl functions
            match &variant.fields {
                Fields::Unit => {
                    return unit::derive_unit_variant_impl(
                        &name, 
                        &variant.ident
                    );
                },
                // If variant is not unit
                Fields::Unnamed(f) => {
                    return tuple::derive_tuple_variant_impl(
                        &name,
                        &variant.ident, 
                        f
                    );
                },
                Fields::Named(f) => {
                    return r#struct::derive_struct_variant_impl(
                        &name, 
                        &variant.ident, 
                        f
                    )
                },
                
            }
        }
    }

    return Error::new(
        name.span(),
        "One field must have the \"zero\" attribute"
    ).to_compile_error();
}

// pub struct DeriveZeroableEnum {
//     name: Ident,
//     variants: Vec<(Ident, Option<Expr>)>
// }

// impl DeriveZeroableEnum {
//     /// Returns the Zeroable implementation for the enum
//     pub fn implementation(self) -> TokenStream {
//         // Initialize zero variant to first variant
//         let mut zero_variant = self.variants[0].0.clone();

//         // Check to see if another variant has a discriminant of zero
//         for (variant_name, disc) in self.variants {
//             // If there is a discriminant
//             if let Some(e) = disc {
//                 // If the expression is literal
//                 match e {
//                     Expr::Lit(le) => {
//                         // If the expression is an integer
//                         match le.lit {
//                             Lit::Int(il) => {
//                                 // If the literal is zero
//                                 if is_zero(il.base10_digits()) {
//                                     zero_variant = variant_name;
//                                     break;
//                                 }
//                             },
//                             _ => {}
//                         }
//                     },
//                     _ => {}
//                 }
//             }
//         }

//         let name = &self.name;

//         quote! {
//             impl safe_pod::Zeroable for #name {
//                 #[inline]
//                 fn zeroed() -> Self {
//                     Self::#zero_variant
//                 }
//             }
//         }.to_token_stream()
//     }
// }

// fn is_zero(str: &str) -> bool {
//     match str {
//         "0u8" | "0u16" | "0u32" | "0u64" | "0u128" |
//         "0i8" | "0i16" | "0i32" | "0i64" | "0i128" |
//         "0" => { true },
//         _ => { false }
//     }
// }