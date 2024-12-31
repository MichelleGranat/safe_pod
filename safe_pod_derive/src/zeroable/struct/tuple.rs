use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, FieldsUnnamed, Ident};

/// Implementation of derive macro for tuple structs
pub fn derive_tuple_struct_impl(name: &Ident, struct_data: &FieldsUnnamed) -> TokenStream {
    // Generate zeroed fields
    let mut zeroed_fields: Vec<TokenStream> = Vec::new();

    for field in &struct_data.unnamed {
        let span = field.span();
        let ty = field.ty.clone();

        zeroed_fields.push(
            quote_spanned! {span => <#ty as safe_pod::Zeroable>::zeroed()}
        );
    }

    // Generate implementation
    quote! {
        impl safe_pod::Zeroable for #name {
            #[inline]
            fn zeroed() -> Self {
                Self(#(#zeroed_fields),* )
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
            impl safe_pod::Zeroable for MyTupleStruct {
                #[inline]
                fn zeroed() -> Self {
                    Self(<u8 as safe_pod::Zeroable>::zeroed())
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
            impl safe_pod::Zeroable for MyTupleStruct {
                #[inline]
                fn zeroed() -> Self {
                    Self(
                        <u8 as safe_pod::Zeroable>::zeroed(),
                        <i32 as safe_pod::Zeroable>::zeroed(),
                        <f64 as safe_pod::Zeroable>::zeroed()
                    )
                }
            }
        }.to_string();

        // Output
        let output = derive_tuple_struct_impl(&name_input, &fields_input).to_string();

        // Test
        assert_eq!(expected_output, output)
    }
}