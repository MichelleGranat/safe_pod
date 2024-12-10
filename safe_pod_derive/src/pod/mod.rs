use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{spanned::Spanned, Data, DeriveInput, Error, Fields, Ident, Type};

/// Implementation of derive macro for Pod trait
pub fn derive_pod_impl(input: DeriveInput) -> TokenStream {
    // If the type that derives Pod is a union
    // return error
    if let Data::Union(d) = &input.data {
        return Error::new(
            d.union_token.span,
            "Union types cannot be Pod"
        ).to_compile_error();
    }

    // If the type that derives Pod is a struct
    if let Data::Struct(d) = &input.data {
        let name = input.ident;
        let mut fields: Vec<(Ident, Type)> = Vec::new();

        match &d.fields {
            Fields::Unit => {
                return Error::new(
                    name.span(),
                    "Unit structs cannot be Pod YET!"
                ).to_compile_error();
            },
            Fields::Unnamed(f) => {
                return Error::new(
                    f.span(),
                    "Structs with unnamed fields cannot derive Pod"
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

        return DerivePodStruct {
            name, 
            fields
        }.implementation();
    }

    // If the type that derives Pod is an enum
    if let Data::Enum(d) = &input.data {
        return Error::new(
            d.enum_token.span,
            "enum types cannot cannot derive Pod YET"
        ).to_compile_error();
    }

    return TokenStream::new();
}

/// A representation of a struct 
/// that derives Pod trait
pub struct DerivePodStruct {
    name: Ident,
    fields: Vec<(Ident, Type)>,
}

impl DerivePodStruct {
    /// Returns the Pod implementation for the struct
    pub fn implementation(self) -> TokenStream {
        // List of field names
        let mut field_names: Vec<TokenStream> = Vec::new();

        // Expression for SIZE
        let mut size_expr: Vec<TokenStream> = Vec::new();

        // Expressions for from_le_bytes
        let mut from_le_bytes_expr: Vec<TokenStream> = Vec::new();

        // Expressions for from_be_bytes
        let mut from_be_bytes_expr: Vec<TokenStream> = Vec::new();

        // Expressions for to_le_bytes
        let mut to_le_bytes_expr: Vec<TokenStream> = Vec::new();

        // Expressions for to_be_bytes
        let mut to_be_bytes_expr: Vec<TokenStream> = Vec::new();

        for (n, t) in self.fields {
            // Add field name
            field_names.push(quote!(#n));

            let ty_span = t.span();
            let name_span = n.span();

            // Add size expr
            size_expr.push(
                quote_spanned!(ty_span => <#t as safe_pod::Pod>::SIZE)
            );

            // Add from_le_bytes
            from_le_bytes_expr.push(
                quote_spanned!(ty_span => 
                    let #n = <#t as safe_pod::Pod>::from_le_bytes(&buffer[bytes..])?;
                    bytes += <#t as safe_pod::Pod>::SIZE;
                )
            );

            // Add from_be_bytes
            from_be_bytes_expr.push(
                quote_spanned!(ty_span => 
                    let #n = <#t as safe_pod::Pod>::from_be_bytes(&buffer[bytes..])?;
                    bytes += <#t as safe_pod::Pod>::SIZE;
                )
            );

            // Add to_le_bytes
            to_le_bytes_expr.push(
                quote_spanned!(name_span => 
                    bytes += safe_pod::Pod::to_le_bytes(&self.#n, &mut buffer[bytes..])?;
                )
            );

            // Add to_be_bytes
            to_be_bytes_expr.push(
                quote_spanned!(name_span => 
                    bytes += safe_pod::Pod::to_be_bytes(&self.#n, &mut buffer[bytes..])?;
                )
            );
        }

        let name = &self.name;

        quote! {
            impl safe_pod::Pod for #name {
                const SIZE: usize = #(#size_expr)+*;

                #[inline]
                fn from_le_bytes(buffer: &[u8]) -> Result<Self, safe_pod::PodError> {
                    if buffer.len() < Self::SIZE {
                        return Err(safe_pod::PodError::OutOfSpace);
                    }

                    let mut bytes = 0usize;

                    #(#from_le_bytes_expr)*

                    Ok(Self { #(#field_names),* })
                }

                #[inline]
                fn from_be_bytes(buffer: &[u8]) -> Result<Self, safe_pod::PodError> {
                    if buffer.len() < Self::SIZE {
                        return Err(safe_pod::PodError::OutOfSpace);
                    }

                    let mut bytes = 0usize;

                    #(#from_be_bytes_expr)*

                    Ok(Self { #(#field_names),* })
                }

                #[inline]
                fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, safe_pod::PodError> {
                    if buffer.len() < Self::SIZE {
                        return Err(safe_pod::PodError::OutOfSpace);
                    }

                    let mut bytes = 0usize;

                    #(#to_le_bytes_expr)*

                    Ok(bytes)
                }

                #[inline]
                fn to_be_bytes(&self, buffer: &mut [u8]) -> Result<usize, safe_pod::PodError> {
                    if buffer.len() < Self::SIZE {
                        return Err(safe_pod::PodError::OutOfSpace);
                    }

                    let mut bytes = 0usize;

                    #(#to_be_bytes_expr)*

                    Ok(bytes)
                }
            }
        }.to_token_stream()
    }
}