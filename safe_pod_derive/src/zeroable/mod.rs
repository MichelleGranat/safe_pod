use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{spanned::Spanned, AttrStyle, Data, DeriveInput, Error, Expr, Fields, Ident, Lit, Type};

use crate::shared::repr::Repr;

/// Implementation of derive macro for Zeroable trait
pub fn derive_zeroable_impl(input: DeriveInput) -> TokenStream {
    // If the type that derives Zeroable is a union
    // return error
    if let Data::Union(d) = &input.data {
        return Error::new(
            d.union_token.span,
            "Union types cannot be Zeroable"
        ).to_compile_error();
    }

    // If the type that derives Zeroable is a Struct
    if let Data::Struct(d) = &input.data {
        // Get name and init fields vector
        let name = input.ident;
        let mut fields: Vec<(Ident, Type)> = Vec::new();

        match &d.fields {
            Fields::Unit => {
                return Error::new(
                    name.span(),
                    "Unit structs cannot be zeroable"
                ).to_compile_error();
            },
            Fields::Unnamed(f) => {
                return Error::new(
                    f.span(),
                    "Structs with unnamed fields cannot derive Zeroable"
                ).to_compile_error();
            },
            Fields::Named(f) => {
                for field in &f.named {
                    fields.push(
                        (field.ident.clone().unwrap(), field.ty.clone())
                    );
                }
            }
        }

        return DeriveZeroableStruct {
            name, 
            fields
        }.implementation();
    }

    // If the type that derives Zeroable is an Enum
    if let Data::Enum(d) = &input.data {
        // Get representation and make sure its supported
        for attribute in input.attrs {
            // Ignore inner attributes
            if let AttrStyle::Inner(_) = attribute.style {
                continue;
            }

            // If path is repr
            if attribute.path().is_ident("repr") {
                // Check that repr is primitive
                if let Err(e) = attribute.parse_nested_meta(
                    |meta| {
                        if Repr::from_path(meta.path.clone()).is_zeroable() {
                            return Ok(())
                        }

                        Err(meta.error("Unsupported representation"))
                    }
                ) {
                    return e.to_compile_error();
                }

                // Break because #[repr] was found
                break;
            }
        }

        // Get name and init variants vector
        let name = input.ident;
        let mut variants: Vec<(Ident, Option<Expr>)> = Vec::new();

        // Go over variants
        for variant in &d.variants {
            // Match fields
            match &variant.fields {
                // If variant is not unit
                Fields::Unnamed(f) => {
                    return Error::new(
                        f.span(),
                        "Non unit-only Enums cannot derive Zeroable YET"
                    ).to_compile_error();
                },
                Fields::Named(f) => {
                    return Error::new(
                        f.span(),
                        "Non unit-only Enums cannot derive Zeroable YET"
                    ).to_compile_error();
                },
                // If variant is unit
                Fields::Unit => {
                    // Get variant name
                    let name = variant.ident.clone();

                    // Get discriminant and push variant to vector
                    if let Some((_, d)) = &variant.discriminant {
                        variants.push((name, Some(d.clone())));
                    } else {
                        variants.push((name, None));
                    }
                },
            }
        }

        return DeriveZeroableEnum {
            name,
            variants
        }.implementation()
    }

    return TokenStream::new();
}

/// A representation of a struct 
/// that derives Zeroable trait
pub struct DeriveZeroableStruct {
    name: Ident,
    fields: Vec<(Ident, Type)>,
}

impl DeriveZeroableStruct {
    /// Returns the Zeroable implementation for the struct
    pub fn implementation(self) -> TokenStream {
        // Generate zeroed fields instructions
        let mut zeroed_fields: Vec<TokenStream> = Vec::new();

        for (n, t) in self.fields {
            let ty_span = t.span();
            // Add zeroing
            zeroed_fields.push(
                quote_spanned!(ty_span => #n: <#t as safe_pod::Zeroable>::zeroed())
            );
        }

        let name = &self.name;

        quote! {
            impl safe_pod::Zeroable for #name {
                #[inline]
                fn zeroed() -> Self {
                    Self { #(#zeroed_fields),* }
                }
            }
        }.to_token_stream()
    }
}

pub struct DeriveZeroableEnum {
    name: Ident,
    variants: Vec<(Ident, Option<Expr>)>
}

impl DeriveZeroableEnum {
    /// Returns the Zeroable implementation for the enum
    pub fn implementation(self) -> TokenStream {
        // Initialize zero variant to first variant
        let mut zero_variant = self.variants[0].0.clone();

        // Check to see if another variant has a discriminant of zero
        for (variant_name, disc) in self.variants {
            // If there is a discriminant
            if let Some(e) = disc {
                // If the expression is literal
                match e {
                    Expr::Lit(le) => {
                        // If the expression is an integer
                        match le.lit {
                            Lit::Int(il) => {
                                // If the literal is zero
                                if is_zero(il.base10_digits()) {
                                    zero_variant = variant_name;
                                    break;
                                }
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }
        }

        let name = &self.name;

        quote! {
            impl safe_pod::Zeroable for #name {
                #[inline]
                fn zeroed() -> Self {
                    Self::#zero_variant
                }
            }
        }.to_token_stream()
    }
}

fn is_zero(str: &str) -> bool {
    match str {
        "0u8" | "0u16" | "0u32" | "0u64" | "0u128" |
        "0i8" | "0i16" | "0i32" | "0i64" | "0i128" |
        "0" => { true },
        _ => { false }
    }
}