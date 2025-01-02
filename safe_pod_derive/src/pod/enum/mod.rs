// Define modules
mod unit;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Attribute, DataEnum, Error, Fields, Ident};

use super::attributes::{EnumAttr, VariantAttr};

/// Implementation of derive macro for enums
pub fn derive_enum_impl(name: Ident, attributes: Vec<Attribute>, enum_data: &DataEnum) -> TokenStream {
    // Get pod attribute
    let pod_attribute = match EnumAttr::from_attributes(&attributes) {
        Ok(pa) => pa,
        Err(e) => match e {
            "not found" => {
                return Error::new(
                    name.span(),
                    "Enums must have the #[pod(...)] attribute"
                ).to_compile_error();
            },
            _ => {
                return Error::new(
                    name.span(),
                    format!("Error while parsing #[pod(...)] attribute: {}", e)
                ).to_compile_error();
            }
        }
    };

    // Get repr attribute
    let repr = match pod_attribute.repr {
        Some(r) => r,
        None => {
            return Error::new(
                name.span(),
                "Enums must have repr in #[pod(repr($type))] set to a type"
            ).to_compile_error();
        }
    };

    // Define variant names
    let mut variant_names: Vec<Ident> = Vec::new();

    // Define from le expressions
    let mut from_le_expressions: Vec<TokenStream> = Vec::new();

    // Define from be expressions
    let mut from_be_expressions: Vec<TokenStream> = Vec::new();

    // Define to le expressions
    let mut to_le_expressions: Vec<TokenStream> = Vec::new();

    // Define to be expressions
    let mut to_be_expressions: Vec<TokenStream> = Vec::new();

    // Generate expressions
    for variant in &enum_data.variants {
        // Match variant type and delegate to appropriate impl functions
        match &variant.fields {
            Fields::Unit => {
                // Check if variant has match attribute and get match expression
                let match_expression = match VariantAttr::from_attributes(&variant.attrs) {
                    Ok(va) =>  match va.match_expr {
                        Some(me) => me,
                        None => {
                                return Error::new(
                                variant.ident.span(),
                                "Variant must have #[pod(match_expr($expression))] attribute"
                            ).to_compile_error();
                        }
                    },
                    Err(e) => {
                        match e {
                            "not found" => {
                                return Error::new(
                                    variant.ident.span(),
                                    "Variants must have #[pod(...)] attribute"
                                ).to_compile_error();
                            },
                            _ => {
                                return Error::new(
                                    variant.ident.span(),
                                    format!("Error while parsing #[pod(...)] attribute: {}", e)
                                ).to_compile_error();
                            }
                        }
                    }
                };

                // Get variant name
                let variant_name = variant.ident.clone();
                variant_names.push(variant_name.clone());

                // Generate from le expression
                from_le_expressions.push(
                    unit::from_le_expression(variant, &match_expression)
                );

                // Generate from be expression
                from_be_expressions.push(
                    unit::from_be_expression(variant, &match_expression)
                );

                // Generate to le expression
                to_le_expressions.push(
                    unit::to_le_expression(variant, &match_expression)
                );

                // Generate to be expression
                to_be_expressions.push(
                    unit::to_be_expression(variant, &match_expression)
                );
            },

            // If variant is not unit
            Fields::Unnamed(_) => {
                return Error::new(
                    variant.span(),
                    "Enums with tuple variants are not supported by Pod derive macro YET!"
                ).to_compile_error();
            },
            Fields::Named(_) => {
                return Error::new(
                    variant.span(),
                    "Enums with struct variants are not supported by Pod derive macro YET!"
                ).to_compile_error();
            }
        }
    }

    
    quote! {
        impl safe_pod::Pod for #name {
            const SIZE: usize = <#repr as safe_pod::Pod>::SIZE ;

            #[inline]
            fn from_le_bytes(buffer: &[u8]) -> Result<Self, safe_pod::PodError> {
                let val = match <#repr as safe_pod::Pod>::from_le_bytes(buffer)? {
                    #(#from_le_expressions)*
                    _ => return Err(safe_pod::PodError::OutOfRange);
                };

                Ok(val)
            }

            #[inline]
            fn from_be_bytes(buffer: &[u8]) -> Result<Self, safe_pod::PodError> {
                let val = match <#repr as safe_pod::Pod>::from_be_bytes(buffer)? {
                    #(#from_be_expressions)*
                    _ => return Err(safe_pod::PodError::OutOfRange);
                };

                Ok(val)
            }

            #[inline]
            fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, safe_pod::PodError> {
                if buffer.len() < Self::SIZE {
                    return Err(safe_pod::PodError::OutOfSpace);
                }

                let val = match &self {
                    #(#to_le_expressions)*
                }

                safe_pod::Pod::to_le_bytes(val, &mut buffer)
            }

            #[inline]
            fn to_be_bytes(&self, buffer: &mut [u8]) -> Result<usize, safe_pod::PodError> {
                if buffer.len() < Self::SIZE {
                    return Err(safe_pod::PodError::OutOfSpace);
                }

                let val = match &self {
                    #(#to_be_expressions)*
                }

                safe_pod::Pod::to_be_bytes(val, &mut buffer)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use syn::{parse2, Data, DeriveInput};

    use super::*;

    #[test]
    fn unit_like() {
        // Define input
        let input_stream = quote! {
            #[pod(repr(u8))]
            enum UnitLike {
                #[pod(match_expr(0))]
                Foo,
                #[pod(match_expr(1))]
                Bar,
            }
        };

        let input = parse2::<DeriveInput>(input_stream).unwrap();
        let name_input = input.ident;
        let attribute_input = input.attrs;
        let enum_data_input = match input.data {
            Data::Enum(ds) => ds,
            _ => unreachable!()
        };

        // Define expected output
        let expected_output = quote! {
            impl safe_pod::Pod for UnitLike {
                const SIZE: usize = <u8 as safe_pod::Pod>::SIZE ;

                #[inline]
                fn from_le_bytes(buffer: &[u8]) -> Result<Self, safe_pod::PodError> {
                    let val = match <u8 as safe_pod::Pod>::from_le_bytes(buffer)? {
                        0 => { Self::Foo },
                        1 => { Self::Bar },
                        _ => return Err(safe_pod::PodError::OutOfRange);
                    };
    
                    Ok(val)
                }

                #[inline]
                fn from_be_bytes(buffer: &[u8]) -> Result<Self, safe_pod::PodError> {
                    let val = match <u8 as safe_pod::Pod>::from_be_bytes(buffer)? {
                        0 => { Self::Foo },
                        1 => { Self::Bar },
                        _ => return Err(safe_pod::PodError::OutOfRange);
                    };
    
                    Ok(val)
                }

                #[inline]
                fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, safe_pod::PodError> {
                    if buffer.len() < Self::SIZE {
                        return Err(safe_pod::PodError::OutOfSpace);
                    }
    
                    let val = match &self {
                        Self::Foo => { 0 },
                        Self::Bar => { 1 },
                    }
    
                    safe_pod::Pod::to_le_bytes(val, &mut buffer)
                }

                #[inline]
                fn to_be_bytes(&self, buffer: &mut [u8]) -> Result<usize, safe_pod::PodError> {
                    if buffer.len() < Self::SIZE {
                        return Err(safe_pod::PodError::OutOfSpace);
                    }
    
                    let val = match &self {
                        Self::Foo => { 0 },
                        Self::Bar => { 1 },
                    }
    
                    safe_pod::Pod::to_be_bytes(val, &mut buffer)
                }
            }
        }.to_string();

        // Output
        let output = derive_enum_impl(name_input, attribute_input, &enum_data_input).to_string();

        // Test
        assert_eq!(expected_output, output)
    }
}