use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, FieldsNamed, Ident};

/// Implementation of derive macro for enums with struct zero variant
pub fn derive_struct_variant_impl(enum_name: &Ident, variant_name: &Ident, variant_data: &FieldsNamed) -> TokenStream {
    // Generate zeroed fields
    let mut zeroed_fields: Vec<TokenStream> = Vec::new();

    for field in &variant_data.named {
        let span = field.span();
        let field_name = match &field.ident {
            Some(i) => i,
            _ => unreachable!()
        };
        let ty = field.ty.clone();

        zeroed_fields.push(
            quote_spanned! {span => #field_name: <#ty as safe_pod::Zeroable>::zeroed()}
        );
    }

    // Generate implementation
    quote! {
        impl safe_pod::Zeroable for #enum_name {
            #[inline]
            fn zeroed() -> Self {
                Self::#variant_name{ #(#zeroed_fields),* }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use syn::{parse2, Data, DeriveInput, Fields};

    use super::*;

    #[test]
    fn struct_variant() {
        // Define input
        let input_stream = quote! {
            enum MyEnum {
                MyUnitVariant,
                MyTupleVariant(u8, f32),
                #[zero]
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
                de.variants.get(2).unwrap().ident.clone()
            }
            _ => unreachable!()
        };
        let variant_input = match input.data {
            Data::Enum(de) => {
                match &de.variants.get(2).unwrap().fields {
                    Fields::Named(f) => f.clone(),
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
                    Self::MyStructVariant{
                        my_field1: <u8 as safe_pod::Zeroable>::zeroed(), 
                        my_field2: <f64 as safe_pod::Zeroable>::zeroed()
                    }
                }
            }
        }.to_string();

        // Output
        let output = derive_struct_variant_impl(&enum_name_input, &variant_name, &variant_input).to_string();

        // Test
        assert_eq!(expected_output, output)
    }
}