use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, FieldsNamed, Ident};

/// Implementation of derive macro for named structs
pub fn derive_named_struct_impl(name: &Ident, struct_data: &FieldsNamed) -> TokenStream {
    // Generate zeroed fields
    let mut zeroed_fields: Vec<TokenStream> = Vec::new();

    for field in &struct_data.named {
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
        impl safe_pod::Zeroable for #name {
            #[inline]
            fn zeroed() -> Self {
                Self { #(#zeroed_fields),* }
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
            impl safe_pod::Zeroable for MyNamedStruct {
                #[inline]
                fn zeroed() -> Self {
                    Self{
                        my_field: <u8 as safe_pod::Zeroable>::zeroed()
                    }
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
            impl safe_pod::Zeroable for MyNamedStruct {
                #[inline]
                fn zeroed() -> Self {
                    Self {
                        my_field1: <u8 as safe_pod::Zeroable>::zeroed(),
                        my_field2: <i32 as safe_pod::Zeroable>::zeroed(),
                        my_field3: <f64 as safe_pod::Zeroable>::zeroed()
                    }
                }
            }
        }.to_string();

        // Output
        let output = derive_named_struct_impl(&name_input, &fields_input).to_string();

        // Test
        assert_eq!(expected_output, output)
    }
}