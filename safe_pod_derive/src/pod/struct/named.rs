use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, FieldsNamed, Ident};

/// Implementation of derive macro for named structs
pub fn derive_named_struct_impl(name: &Ident, struct_data: &FieldsNamed) -> TokenStream {
    // Define size expressions
    let mut size_expressions: Vec<TokenStream> = Vec::new();

    // Define field names
    let mut field_names: Vec<Ident> = Vec::new();

    // Define from le expressions
    let mut from_le_expressions: Vec<TokenStream> = Vec::new();

    // Define from be expressions
    let mut from_be_expressions: Vec<TokenStream> = Vec::new();

    // Define to le expressions
    let mut to_le_expressions: Vec<TokenStream> = Vec::new();

    // Define to be expressions
    let mut to_be_expressions: Vec<TokenStream> = Vec::new();

    // Go over fields and generate expressions
    for field in &struct_data.named {
        let span = field.span();
        let ty = field.ty.clone();
        
        // Generate size expression
        size_expressions.push(
            quote_spanned! {span => <#ty as safe_pod::Pod>::SIZE}
        );

        // Get field name
        let field_name = field.ident.clone().unwrap();
        field_names.push(field_name.clone());

        // Generate from le expression
        from_le_expressions.push(
            quote_spanned! {span => 
                let #field_name = <#ty as safe_pod::Pod>::from_le_bytes(&buffer[offset..])?;
                offset += <#ty as safe_pod::Pod>::SIZE;
            }
        );

        // Generate from be expression
        from_be_expressions.push(
            quote_spanned! {span => 
                let #field_name = <#ty as safe_pod::Pod>::from_be_bytes(&buffer[offset..])?;
                offset += <#ty as safe_pod::Pod>::SIZE;
            }
        );

        // Generate to le expression
        to_le_expressions.push(
            quote_spanned! {span => 
                offset += safe_pod::Pod::to_le_bytes(&self.#field_name, &mut buffer[offset..])?;
            }
        );
        
        // Generate to be expression
        to_be_expressions.push(
            quote_spanned! {span => 
                offset += safe_pod::Pod::to_be_bytes(&self.#field_name, &mut buffer[offset..])?;
            }
        );
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

                Ok(Self{ #(#field_names),* })
            }

            #[inline]
            #[allow(unused_assignments)]
            fn from_be_bytes(buffer: &[u8]) -> Result<Self, safe_pod::PodError> {
                if buffer.len() < Self::SIZE {
                    return Err(safe_pod::PodError::OutOfSpace);
                }

                let mut offset = 0usize;

                #(#from_be_expressions)*

                Ok(Self{ #(#field_names),* })
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
    fn named_struct_one_field() {
        // Define input
        let input_stream = quote! {
            struct MyNamedStruct{
                my_field: u8
            }
        };

        let input = parse2::<DeriveInput>(input_stream).unwrap();
        let name_input = input.ident;
        let fields_input = match input.data {
            Data::Struct(ds) => {
                match ds.fields {
                    Fields::Named(f) => {
                        f
                    },
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        };

        // Define expected output
        let expected_output = quote! {
            impl safe_pod::Pod for MyNamedStruct {
                const SIZE: usize = <u8 as safe_pod::Pod>::SIZE ;

                #[inline]
                #[allow(unused_assignments)]
                fn from_le_bytes(buffer: &[u8]) -> Result<Self, safe_pod::PodError> {
                    if buffer.len() < Self::SIZE {
                        return Err(safe_pod::PodError::OutOfSpace);
                    }

                    let mut offset = 0usize;

                    let my_field = <u8 as safe_pod::Pod>::from_le_bytes(&buffer[offset..])?;
                    offset += <u8 as safe_pod::Pod>::SIZE;

                    Ok(Self{ my_field })
                }

                #[inline]
                #[allow(unused_assignments)]
                fn from_be_bytes(buffer: &[u8]) -> Result<Self, safe_pod::PodError> {
                    if buffer.len() < Self::SIZE {
                        return Err(safe_pod::PodError::OutOfSpace);
                    }

                    let mut offset = 0usize;

                    let my_field = <u8 as safe_pod::Pod>::from_be_bytes(&buffer[offset..])?;
                    offset += <u8 as safe_pod::Pod>::SIZE;

                    Ok(Self{ my_field })
                }

                #[inline]
                fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, safe_pod::PodError> {
                    if buffer.len() < Self::SIZE {
                        return Err(safe_pod::PodError::OutOfSpace);
                    }

                    let mut offset = 0usize;

                    offset += safe_pod::Pod::to_le_bytes(&self.my_field, &mut buffer[offset..])?;

                    Ok(offset)
                }

                #[inline]
                fn to_be_bytes(&self, buffer: &mut [u8]) -> Result<usize, safe_pod::PodError> {
                    if buffer.len() < Self::SIZE {
                        return Err(safe_pod::PodError::OutOfSpace);
                    }

                    let mut offset = 0usize;

                    offset += safe_pod::Pod::to_be_bytes(&self.my_field, &mut buffer[offset..])?;

                    Ok(offset)
                }
            }
        }.to_string();

        // Output
        let output = derive_named_struct_impl(&name_input, &fields_input).to_string();

        // Test
        assert_eq!(expected_output, output)
    }

    #[test]
    fn named_struct_multiple_fields() {
        // Define input
        let input_stream = quote! {
            struct MyNamedStruct{
                my_field1: u8, 
                my_field2: i32,
                my_field3: f64
            }
        };

        let input = parse2::<DeriveInput>(input_stream).unwrap();
        let name_input = input.ident;
        let fields_input = match input.data {
            Data::Struct(ds) => {
                match ds.fields {
                    Fields::Named(f) => {
                        f
                    },
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        };

        // Define expected output
        let expected_output = quote! {
            impl safe_pod::Pod for MyNamedStruct {
                const SIZE: usize = <u8 as safe_pod::Pod>::SIZE + <i32 as safe_pod::Pod>::SIZE + <f64 as safe_pod::Pod>::SIZE ;

                #[inline]
                #[allow(unused_assignments)]
                fn from_le_bytes(buffer: &[u8]) -> Result<Self, safe_pod::PodError> {
                    if buffer.len() < Self::SIZE {
                        return Err(safe_pod::PodError::OutOfSpace);
                    }

                    let mut offset = 0usize;

                    let my_field1 = <u8 as safe_pod::Pod>::from_le_bytes(&buffer[offset..])?;
                    offset += <u8 as safe_pod::Pod>::SIZE;

                    let my_field2 = <i32 as safe_pod::Pod>::from_le_bytes(&buffer[offset..])?;
                    offset += <i32 as safe_pod::Pod>::SIZE;

                    let my_field3 = <f64 as safe_pod::Pod>::from_le_bytes(&buffer[offset..])?;
                    offset += <f64 as safe_pod::Pod>::SIZE;

                    Ok(Self{ my_field1, my_field2, my_field3 })
                }

                #[inline]
                #[allow(unused_assignments)]
                fn from_be_bytes(buffer: &[u8]) -> Result<Self, safe_pod::PodError> {
                    if buffer.len() < Self::SIZE {
                        return Err(safe_pod::PodError::OutOfSpace);
                    }

                    let mut offset = 0usize;

                    let my_field1 = <u8 as safe_pod::Pod>::from_be_bytes(&buffer[offset..])?;
                    offset += <u8 as safe_pod::Pod>::SIZE;

                    let my_field2 = <i32 as safe_pod::Pod>::from_be_bytes(&buffer[offset..])?;
                    offset += <i32 as safe_pod::Pod>::SIZE;

                    let my_field3 = <f64 as safe_pod::Pod>::from_be_bytes(&buffer[offset..])?;
                    offset += <f64 as safe_pod::Pod>::SIZE;

                    Ok(Self{ my_field1, my_field2, my_field3 })
                }

                #[inline]
                fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, safe_pod::PodError> {
                    if buffer.len() < Self::SIZE {
                        return Err(safe_pod::PodError::OutOfSpace);
                    }

                    let mut offset = 0usize;

                    offset += safe_pod::Pod::to_le_bytes(&self.my_field1, &mut buffer[offset..])?;
                    offset += safe_pod::Pod::to_le_bytes(&self.my_field2, &mut buffer[offset..])?;
                    offset += safe_pod::Pod::to_le_bytes(&self.my_field3, &mut buffer[offset..])?;

                    Ok(offset)
                }

                #[inline]
                fn to_be_bytes(&self, buffer: &mut [u8]) -> Result<usize, safe_pod::PodError> {
                    if buffer.len() < Self::SIZE {
                        return Err(safe_pod::PodError::OutOfSpace);
                    }

                    let mut offset = 0usize;

                    offset += safe_pod::Pod::to_be_bytes(&self.my_field1, &mut buffer[offset..])?;
                    offset += safe_pod::Pod::to_be_bytes(&self.my_field2, &mut buffer[offset..])?;
                    offset += safe_pod::Pod::to_be_bytes(&self.my_field3, &mut buffer[offset..])?;

                    Ok(offset)
                }
            }
        }.to_string();

        // Output
        let output = derive_named_struct_impl(&name_input, &fields_input).to_string();

        // Test
        assert_eq!(expected_output, output)
    }
}