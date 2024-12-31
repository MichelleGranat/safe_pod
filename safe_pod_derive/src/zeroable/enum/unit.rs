use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

/// Implementation of derive macro for enums with unit zero variant
pub fn derive_unit_variant_impl(enum_name: &Ident, variant_name: &Ident) -> TokenStream {
    // Implement Zeroable for unit variant
    quote! {
        impl safe_pod::Zeroable for #enum_name {
            #[inline]
            fn zeroed() -> Self {
                Self::#variant_name
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::{parse2, Data, DeriveInput};

    use super::*;

    #[test]
    fn unit_variant() {
        // Define input
        let input_stream = quote! {
            enum MyEnum {
                #[zero]
                MyUnitVariant,
                MyTupleVariant(u8, f32),
                MyStructVariant {
                    my_field1: u8,
                    my_field2: f64
                },
            }
        };

        let input = parse2::<DeriveInput>(input_stream).unwrap();
        let enum_name_input = input.ident;
        let variant_name = match input.data {
            Data::Enum(de) => {
                de.variants.first().unwrap().ident.clone()
            }
            _ => unreachable!()
        };

        // Define expected output
        let expected_output = quote! {
            impl safe_pod::Zeroable for MyEnum {
                #[inline]
                fn zeroed() -> Self {
                    Self::MyUnitVariant
                }
            }
        }.to_string();

        // Output
        let output = derive_unit_variant_impl(&enum_name_input, &variant_name).to_string();

        // Test
        assert_eq!(expected_output, output)
    }
}