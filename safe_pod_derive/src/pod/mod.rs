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

// /// A representation of a struct 
// /// that derives Pod trait
// pub struct DerivePodStruct {
//     name: Ident,
//     fields: Vec<(Ident, Type)>,
// }

// impl DerivePodStruct {
//     /// Returns the Pod implementation for the struct
//     pub fn implementation(self) -> TokenStream {
//         // List of field names
//         let mut field_names: Vec<TokenStream> = Vec::new();

//         // Expression for SIZE
//         let mut size_expr: Vec<TokenStream> = Vec::new();

//         // Expressions for from_le_bytes
//         let mut from_le_bytes_expr: Vec<TokenStream> = Vec::new();

//         // Expressions for from_be_bytes
//         let mut from_be_bytes_expr: Vec<TokenStream> = Vec::new();

//         // Expressions for to_le_bytes
//         let mut to_le_bytes_expr: Vec<TokenStream> = Vec::new();

//         // Expressions for to_be_bytes
//         let mut to_be_bytes_expr: Vec<TokenStream> = Vec::new();

//         for (n, t) in self.fields {
//             // Add field name
//             field_names.push(quote!(#n));

//             let ty_span = t.span();
//             let name_span = n.span();

//             // Add size expr
//             size_expr.push(
//                 quote_spanned!(ty_span => <#t as safe_pod::Pod>::SIZE)
//             );

//             // Add from_le_bytes
//             from_le_bytes_expr.push(
//                 quote_spanned!(ty_span => 
//                     let #n = <#t as safe_pod::Pod>::from_le_bytes(&buffer[bytes..])?;
//                     bytes += <#t as safe_pod::Pod>::SIZE;
//                 )
//             );

//             // Add from_be_bytes
//             from_be_bytes_expr.push(
//                 quote_spanned!(ty_span => 
//                     let #n = <#t as safe_pod::Pod>::from_be_bytes(&buffer[bytes..])?;
//                     bytes += <#t as safe_pod::Pod>::SIZE;
//                 )
//             );

//             // Add to_le_bytes
//             to_le_bytes_expr.push(
//                 quote_spanned!(name_span => 
//                     bytes += safe_pod::Pod::to_le_bytes(&self.#n, &mut buffer[bytes..])?;
//                 )
//             );

//             // Add to_be_bytes
//             to_be_bytes_expr.push(
//                 quote_spanned!(name_span => 
//                     bytes += safe_pod::Pod::to_be_bytes(&self.#n, &mut buffer[bytes..])?;
//                 )
//             );
//         }

//         let name = &self.name;

//         quote! {
//             impl safe_pod::Pod for #name {
//                 const SIZE: usize = #(#size_expr)+*;

//                 #[inline]
//                 fn from_le_bytes(buffer: &[u8]) -> Result<Self, safe_pod::PodError> {
//                     if buffer.len() < Self::SIZE {
//                         return Err(safe_pod::PodError::OutOfSpace);
//                     }

//                     let mut bytes = 0usize;

//                     #(#from_le_bytes_expr)*

//                     Ok(Self { #(#field_names),* })
//                 }

//                 #[inline]
//                 fn from_be_bytes(buffer: &[u8]) -> Result<Self, safe_pod::PodError> {
//                     if buffer.len() < Self::SIZE {
//                         return Err(safe_pod::PodError::OutOfSpace);
//                     }

//                     let mut bytes = 0usize;

//                     #(#from_be_bytes_expr)*

//                     Ok(Self { #(#field_names),* })
//                 }

//                 #[inline]
//                 fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, safe_pod::PodError> {
//                     if buffer.len() < Self::SIZE {
//                         return Err(safe_pod::PodError::OutOfSpace);
//                     }

//                     let mut bytes = 0usize;

//                     #(#to_le_bytes_expr)*

//                     Ok(bytes)
//                 }

//                 #[inline]
//                 fn to_be_bytes(&self, buffer: &mut [u8]) -> Result<usize, safe_pod::PodError> {
//                     if buffer.len() < Self::SIZE {
//                         return Err(safe_pod::PodError::OutOfSpace);
//                     }

//                     let mut bytes = 0usize;

//                     #(#to_be_bytes_expr)*

//                     Ok(bytes)
//                 }
//             }
//         }.to_token_stream()
//     }
// }

// pub struct DerivePodEnum {
//     repr: Repr,
//     name: Ident,
//     variants: Vec<(Ident, Span)>
// }

// impl DerivePodEnum {
//     /// Returns the Zeroable implementation for the enum
//     pub fn implementation(self) -> TokenStream {
//         // Get basic info
//         let name = &self.name;
//         let size_expr = Literal::from_str(self.repr.size()).unwrap(); // Can't fail
//         let repr_ident: TokenStream = self.repr.ident().parse().unwrap(); // Can't fail
//         let repr_byte_array_expr = self.repr.byte_array("buffer");

//         // Initialize zero variant to first variant
//         let mut variant_match_exprs: Vec<TokenStream> = Vec::new();

//         // Check to see if another variant has a discriminant of zero
//         for (variant, span) in self.variants {
//             variant_match_exprs.push(quote_spanned! {span =>
//                 #variant as #repr_ident => { Ok(Self::#variant) }
//             });
//         }

//         quote! {
//             impl safe_pod::Pod for #name {
//                 const SIZE: usize = #size_expr;

//                 #[inline]
//                 fn from_le_bytes(buffer: &[u8]) -> Result<Self, safe_pod::PodError> {
//                     if buffer.len() < Self::SIZE {
//                         return Err(safe_pod::PodError::OutOfSpace);
//                     }

//                     let num = #repr_ident::from_le_bytes(#repr_byte_array_expr);

//                     match num {
//                         #(#variant_match_exprs),*
//                         _ => { Err(safe_pod::PodError::OutOfRange) }
//                     }
//                 }

//                 #[inline]
//                 fn from_be_bytes(buffer: &[u8]) -> Result<Self, safe_pod::PodError> {
//                     if buffer.len() < Self::SIZE {
//                         return Err(safe_pod::PodError::OutOfSpace);
//                     }

//                     let num = #repr_ident::from_be_bytes(#repr_byte_array_expr);

//                     match num {
//                         #(#variant_match_exprs),*
//                         _ => { Err(safe_pod::PodError::OutOfRange) }
//                     }
//                 }

//                 #[inline]
//                 fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, safe_pod::PodError> {
//                     if buffer.len() < Self::SIZE {
//                         return Err(safe_pod::PodError::OutOfSpace);
//                     }

//                     let mut bytes = (slef as #repr_ident).to_le_bytes();

//                     buffer.copy_from_slice(&bytes);

//                     Ok(Self::SIZE)
//                 }

//                 #[inline]
//                 fn to_be_bytes(&self, buffer: &mut [u8]) -> Result<usize, safe_pod::PodError> {
//                     if buffer.len() < Self::SIZE {
//                         return Err(safe_pod::PodError::OutOfSpace);
//                     }

//                     let mut bytes = (slef as #repr_ident).to_be_bytes();

//                     buffer.copy_from_slice(&bytes);

//                     Ok(Self::SIZE)
//                 }
//             }
//         }.to_token_stream()
//     }
// }
