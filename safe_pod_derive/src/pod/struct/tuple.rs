use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, FieldsUnnamed, Ident, Index};

/// Implementation of derive macro for tuple structs
pub fn derive_tuple_struct_impl(name: &Ident, struct_data: &FieldsUnnamed) -> TokenStream {
    // Define size expressions
    let mut size_expressions: Vec<TokenStream> = Vec::new();

    // Define temporary variable names
    let mut temp_names: Vec<Ident> = Vec::new();

    // Define from le expressions
    let mut from_le_expressions: Vec<TokenStream> = Vec::new();

    // Define from be expressions
    let mut from_be_expressions: Vec<TokenStream> = Vec::new();

    // Define to le expressions
    let mut to_le_expressions: Vec<TokenStream> = Vec::new();

    // Define to be expressions
    let mut to_be_expressions: Vec<TokenStream> = Vec::new();

    // Go over fields and generate expressions
    let mut n = 0usize;
    for field in &struct_data.unnamed {
        let span = field.span();
        let ty = field.ty.clone();
        
        // Generate size expression
        size_expressions.push(
            quote_spanned! {span => <#ty as safe_pod::Pod>::SIZE}
        );

        // Generate temporary name
        let temp_name = Ident::new(&format!("temp_{}", n), span);
        temp_names.push(temp_name.clone());

        // Generate from le expression
        from_le_expressions.push(
            quote_spanned! {span => 
                let #temp_name = <#ty as safe_pod::Pod>::from_le_bytes(&buffer[offset..])?;
                offset += <#ty as safe_pod::Pod>::SIZE;
            }
        );

        // Generate from be expression
        from_be_expressions.push(
            quote_spanned! {span => 
                let #temp_name = <#ty as safe_pod::Pod>::from_be_bytes(&buffer[offset..])?;
                offset += <#ty as safe_pod::Pod>::SIZE;
            }
        );

        // Generate to le expression
        let field_index = Index::from(n);
        to_le_expressions.push(
            quote_spanned! {span => 
                offset += safe_pod::Pod::to_le_bytes(&self.#field_index, &mut buffer[offset..])?;
            }
        );
        
        // Generate to be expression
        let field_index = Index::from(n);
        to_be_expressions.push(
            quote_spanned! {span => 
                offset += safe_pod::Pod::to_be_bytes(&self.#field_index, &mut buffer[offset..])?;
            }
        );

        // Incement n
        n += 1;
    }

    // Generate implementation
    quote! {
        impl safe_pod::Pod for #name {
            const SIZE: usize = #(#size_expressions)+* ;

            #[inline]
            #[allow(unused_assignments)]
            fn from_le_bytes(buffer: &[u8]) -> Result<Self, safe_pod::PodError> {
                if buffer.len() < Self::SIZE {
                    return Err(safe_pod::PodError::OutOfSpace);
                }

                let mut offset = 0usize;

                #(#from_le_expressions)*

                Ok(Self(#(#temp_names),*))
            }

            #[inline]
            #[allow(unused_assignments)]
            fn from_be_bytes(buffer: &[u8]) -> Result<Self, safe_pod::PodError> {
                if buffer.len() < Self::SIZE {
                    return Err(safe_pod::PodError::OutOfSpace);
                }

                let mut offset = 0usize;

                #(#from_be_expressions)*

                Ok(Self(#(#temp_names),*))
            }

            #[inline]
            fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, safe_pod::PodError> {
                if buffer.len() < Self::SIZE {
                    return Err(safe_pod::PodError::OutOfSpace);
                }

                let mut offset = 0usize;

                #(#to_le_expressions)*

                Ok(offset)
            }

            #[inline]
            fn to_be_bytes(&self, buffer: &mut [u8]) -> Result<usize, safe_pod::PodError> {
                if buffer.len() < Self::SIZE {
                    return Err(safe_pod::PodError::OutOfSpace);
                }

                let mut offset = 0usize;

                #(#to_be_expressions)*

                Ok(offset)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::{parse2, Data, DeriveInput, Fields};

    use super::*;

    #[test]
    fn tuple_struct_one_field() {
        // Define input
        let input_stream = quote! {
            struct MyTupleStruct(u8);
        };

        let input = parse2::<DeriveInput>(input_stream).unwrap();
        let name_input = input.ident;
        let fields_input = match input.data {
            Data::Struct(ds) => {
                match ds.fields {
                    Fields::Unnamed(f) => {
                        f
                    },
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        };

        // Define expected output
        let expected_output = quote! {
            impl safe_pod::Pod for MyTupleStruct {
                const SIZE: usize = <u8 as safe_pod::Pod>::SIZE ;

                #[inline]
                #[allow(unused_assignments)]
                fn from_le_bytes(buffer: &[u8]) -> Result<Self, safe_pod::PodError> {
                    if buffer.len() < Self::SIZE {
                        return Err(safe_pod::PodError::OutOfSpace);
                    }

                    let mut offset = 0usize;

                    let temp_0 = <u8 as safe_pod::Pod>::from_le_bytes(&buffer[offset..])?;
                    offset += <u8 as safe_pod::Pod>::SIZE;

                    Ok(Self(temp_0))
                }

                #[inline]
                #[allow(unused_assignments)]
                fn from_be_bytes(buffer: &[u8]) -> Result<Self, safe_pod::PodError> {
                    if buffer.len() < Self::SIZE {
                        return Err(safe_pod::PodError::OutOfSpace);
                    }

                    let mut offset = 0usize;

                    let temp_0 = <u8 as safe_pod::Pod>::from_be_bytes(&buffer[offset..])?;
                    offset += <u8 as safe_pod::Pod>::SIZE;

                    Ok(Self(temp_0))
                }

                #[inline]
                fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, safe_pod::PodError> {
                    if buffer.len() < Self::SIZE {
                        return Err(safe_pod::PodError::OutOfSpace);
                    }

                    let mut offset = 0usize;

                    offset += safe_pod::Pod::to_le_bytes(&self.0, &mut buffer[offset..])?;

                    Ok(offset)
                }

                #[inline]
                fn to_be_bytes(&self, buffer: &mut [u8]) -> Result<usize, safe_pod::PodError> {
                    if buffer.len() < Self::SIZE {
                        return Err(safe_pod::PodError::OutOfSpace);
                    }

                    let mut offset = 0usize;

                    offset += safe_pod::Pod::to_be_bytes(&self.0, &mut buffer[offset..])?;

                    Ok(offset)
                }
            }
        }.to_string();

        // Output
        let output = derive_tuple_struct_impl(&name_input, &fields_input).to_string();

        // Test
        assert_eq!(expected_output, output)
    }

    #[test]
    fn tuple_struct_multiple_fields() {
        // Define input
        let input_stream = quote! {
            struct MyTupleStruct(u8, i32, f64);
        };

        let input = parse2::<DeriveInput>(input_stream).unwrap();
        let name_input = input.ident;
        let fields_input = match input.data {
            Data::Struct(ds) => {
                match ds.fields {
                    Fields::Unnamed(f) => {
                        f
                    },
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        };

        // Define expected output
        let expected_output = quote! {
            impl safe_pod::Pod for MyTupleStruct {
                const SIZE: usize = <u8 as safe_pod::Pod>::SIZE + <i32 as safe_pod::Pod>::SIZE + <f64 as safe_pod::Pod>::SIZE ;

                #[inline]
                #[allow(unused_assignments)]
                fn from_le_bytes(buffer: &[u8]) -> Result<Self, safe_pod::PodError> {
                    if buffer.len() < Self::SIZE {
                        return Err(safe_pod::PodError::OutOfSpace);
                    }

                    let mut offset = 0usize;

                    let temp_0 = <u8 as safe_pod::Pod>::from_le_bytes(&buffer[offset..])?;
                    offset += <u8 as safe_pod::Pod>::SIZE;

                    let temp_1 = <i32 as safe_pod::Pod>::from_le_bytes(&buffer[offset..])?;
                    offset += <i32 as safe_pod::Pod>::SIZE;

                    let temp_2 = <f64 as safe_pod::Pod>::from_le_bytes(&buffer[offset..])?;
                    offset += <f64 as safe_pod::Pod>::SIZE;

                    Ok(Self(temp_0, temp_1, temp_2))
                }

                #[inline]
                #[allow(unused_assignments)]
                fn from_be_bytes(buffer: &[u8]) -> Result<Self, safe_pod::PodError> {
                    if buffer.len() < Self::SIZE {
                        return Err(safe_pod::PodError::OutOfSpace);
                    }

                    let mut offset = 0usize;

                    let temp_0 = <u8 as safe_pod::Pod>::from_be_bytes(&buffer[offset..])?;
                    offset += <u8 as safe_pod::Pod>::SIZE;

                    let temp_1 = <i32 as safe_pod::Pod>::from_be_bytes(&buffer[offset..])?;
                    offset += <i32 as safe_pod::Pod>::SIZE;

                    let temp_2 = <f64 as safe_pod::Pod>::from_be_bytes(&buffer[offset..])?;
                    offset += <f64 as safe_pod::Pod>::SIZE;

                    Ok(Self(temp_0, temp_1, temp_2))
                }

                #[inline]
                fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, safe_pod::PodError> {
                    if buffer.len() < Self::SIZE {
                        return Err(safe_pod::PodError::OutOfSpace);
                    }

                    let mut offset = 0usize;

                    offset += safe_pod::Pod::to_le_bytes(&self.0, &mut buffer[offset..])?;
                    offset += safe_pod::Pod::to_le_bytes(&self.1, &mut buffer[offset..])?;
                    offset += safe_pod::Pod::to_le_bytes(&self.2, &mut buffer[offset..])?;

                    Ok(offset)
                }

                #[inline]
                fn to_be_bytes(&self, buffer: &mut [u8]) -> Result<usize, safe_pod::PodError> {
                    if buffer.len() < Self::SIZE {
                        return Err(safe_pod::PodError::OutOfSpace);
                    }

                    let mut offset = 0usize;

                    offset += safe_pod::Pod::to_be_bytes(&self.0, &mut buffer[offset..])?;
                    offset += safe_pod::Pod::to_be_bytes(&self.1, &mut buffer[offset..])?;
                    offset += safe_pod::Pod::to_be_bytes(&self.2, &mut buffer[offset..])?;

                    Ok(offset)
                }
            }
        }.to_string();

        // Output
        let output = derive_tuple_struct_impl(&name_input, &fields_input).to_string();

        // Test
        assert_eq!(expected_output, output)
    }
}