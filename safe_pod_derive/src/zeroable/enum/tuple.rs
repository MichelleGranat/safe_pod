use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, FieldsUnnamed, Ident};

/// Implementation of derive macro for enums with tuple zero variant
pub fn derive_tuple_variant_impl(enum_name: &Ident, variant_name: &Ident, variant_data: &FieldsUnnamed) -> TokenStream {
    // Generate zeroed fields
    let mut zeroed_fields: Vec<TokenStream> = Vec::new();

    for field in &variant_data.unnamed {
        let span = field.span();
        let ty = field.ty.clone();

        zeroed_fields.push(
            quote_spanned! {span => <#ty as safe_pod::Zeroable>::zeroed()}
        );
    }

    // Generate implementation
    quote! {
        impl safe_pod::Zeroable for #enum_name {
            #[inline]
            fn zeroed() -> Self {
                Self::#variant_name(#(#zeroed_fields),* )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::{parse2, Data, DeriveInput, Fields};

    use super::*;

    #[test]
    fn tuple_variant() {
        // Define input
        let input_stream = quote! {
            enum MyEnum {
                MyUnitVariant,
                #[zero]
                MyTupleVariant(u8, f32),
                MyStructVariant {
                    my_field1: u8,
                    my_field2: f64
                },
            }
        };

        let input = parse2::<DeriveInput>(input_stream).unwrap();
        let enum_name_input = input.ident;
        let variant_name = match &input.data {
            Data::Enum(de) => {
                de.variants.get(1).unwrap().ident.clone()
            }
            _ => unreachable!()
        };
        let variant_input = match input.data {
            Data::Enum(de) => {
                match &de.variants.get(1).unwrap().fields {
                    Fields::Unnamed(f) => f.clone(),
                    _ => unreachable!()
                }
            }
            _ => unreachable!()
        };

        // Define expected output
        let expected_output = quote! {
            impl safe_pod::Zeroable for MyEnum {
                #[inline]
                fn zeroed() -> Self {
                    Self::MyTupleVariant(
                        <u8 as safe_pod::Zeroable>::zeroed(), 
                        <f32 as safe_pod::Zeroable>::zeroed()
                    )
                }
            }
        }.to_string();

        // Output
        let output = derive_tuple_variant_impl(&enum_name_input, &variant_name, &variant_input).to_string();

        // Test
        assert_eq!(expected_output, output)
    }
}