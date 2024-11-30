use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{spanned::Spanned, Data, DeriveInput, Error, Fields, Ident, Type};

/// Implementation of derive macro for Zeroable trait
pub fn derive_zeroable_impl(input: DeriveInput) -> TokenStream {
    // If the type that derives zeroable is a union
    // return error
    if let Data::Union(d) = &input.data {
        return Error::new(
            d.union_token.span,
            "union types cannot be Zeroable"
        ).to_compile_error();
    }

    // If the type that derives zeroable is a struct
    if let Data::Struct(d) = &input.data {
        let name = input.ident;
        let mut fields: Vec<(Ident, Type)> = Vec::new();

        match &d.fields {
            Fields::Unit => {
                return Error::new(
                    name.span(),
                    "Unit structs are not zeroable"
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

    // If the type that derives zeroable is an enum
    if let Data::Enum(d) = &input.data {
        return Error::new(
            d.enum_token.span,
            "enum types cannot cannot derive Zeroable"
        ).to_compile_error();
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
    pub fn implementation(&self) -> TokenStream {
        // Generate zeroed fields instructions
        let mut zeroed_fields: Vec<TokenStream> = Vec::new();

        for (n, t) in &self.fields {
            let ty_span = t.span();
            // Add zeroing
            zeroed_fields.push(
                quote_spanned!(ty_span => #n: #t::zeroed())
            );
        }

        let name = &self.name;

        quote! {
            impl Zeroable for #name {
                fn zeroed() -> Self {
                    Self {
                        #(#zeroed_fields)*,
                    }
                }
            }
        }.to_token_stream()
    }
}
