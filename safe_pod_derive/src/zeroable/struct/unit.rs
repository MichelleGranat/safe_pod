use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

/// Implementation of derive macro for unit structs
pub fn derive_unit_struct_impl(name: &Ident) -> TokenStream {
    // Implement Zeroable for unit struct
    quote! {
        impl safe_pod::Zeroable for #name {
            #[inline]
            fn zeroed() -> Self {
                Self { }
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
            impl safe_pod::Zeroable for MyUnitStruct {
                #[inline]
                fn zeroed() -> Self {
                    Self { }
                }
            }
        }.to_string();

        // Output
        let output = derive_unit_struct_impl(&input).to_string();

        // Test
        assert_eq!(expected_output, output)
    }
}