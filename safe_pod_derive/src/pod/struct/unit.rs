use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

/// Implementation of derive macro for unit structs
pub fn derive_unit_struct_impl(name: &Ident) -> TokenStream {
    // Implement Pod for unit struct
    quote! {
        impl safe_pod::Pod for #name {
            const SIZE: usize = 0;

            #[inline]
            fn from_le_bytes(buffer: &[u8]) -> Result<Self, safe_pod::PodError> {
                Ok(Self { })
            }

            #[inline]
            fn from_be_bytes(buffer: &[u8]) -> Result<Self, safe_pod::PodError> {
                Ok(Self { })
            }

            #[inline]
            fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, safe_pod::PodError> {
                Ok(0)
            }

            #[inline]
            fn to_be_bytes(&self, buffer: &mut [u8]) -> Result<usize, safe_pod::PodError> {
                Ok(0)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::{parse2, DeriveInput};

    use super::*;

    #[test]
    fn unit_struct() {
        // Define input
        let input_stream = quote! {
            struct MyUnitStruct;
        };

        let input = parse2::<DeriveInput>(input_stream).unwrap().ident;

        // Define expected output
        let expected_output = quote! {
            impl safe_pod::Pod for MyUnitStruct {
                const SIZE: usize = 0;

                #[inline]
                fn from_le_bytes(buffer: &[u8]) -> Result<Self, safe_pod::PodError> {
                    Ok(Self { })
                }

                #[inline]
                fn from_be_bytes(buffer: &[u8]) -> Result<Self, safe_pod::PodError> {
                    Ok(Self { })
                }

                #[inline]
                fn to_le_bytes(&self, buffer: &mut [u8]) -> Result<usize, safe_pod::PodError> {
                    Ok(0)
                }

                #[inline]
                fn to_be_bytes(&self, buffer: &mut [u8]) -> Result<usize, safe_pod::PodError> {
                    Ok(0)
                }
            }
        }.to_string();

        // Output
        let output = derive_unit_struct_impl(&input).to_string();

        // Test
        assert_eq!(expected_output, output)
    }
}