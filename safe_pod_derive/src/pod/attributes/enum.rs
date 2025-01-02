use syn::{Attribute, Type};

use super::utils;

/// `pod` attribute struct for an enum
#[derive(Debug, PartialEq, Eq)]
pub struct EnumAttr {
    pub repr: Option<Type>
}

impl EnumAttr {
    // Extracts EnumAttr from an attribute vector
    pub fn from_attributes(attributes: &Vec<Attribute>) -> Result<Self, &'static str> {
        // Define fields
        let mut repr: Option<Type> = None;

        // Get inner attributes
        let attrs = match utils::get_pod(attributes) {
            Ok(a) => a,
            Err(e) => return Err(e)
        };

        // Get attributes
        repr = match utils::get_repr(&attrs) {
            Ok(r) => Some(r),
            Err(e) => match e {
                "not found" => None,
                _ => return Err(e)
            }
        };

        return Ok( EnumAttr { repr });
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use syn::{parse2, DeriveInput, Ident, Path, TypePath};
    use quote::quote;

    use super::*;

    #[test]
    fn enum_attribute_success() {
        // Define input
        let input_stream = quote! {
            #[pod(repr(u16), other(), other2 = val)]
            struct Foo;
        };

        let input = parse2::<DeriveInput>(input_stream).unwrap().attrs;

        // Define expected output
        let expected_output = EnumAttr {
            repr: Some(Type::Path(
                TypePath {
                    qself: None,
                    path: Path::from(Ident::new("u16", Span::call_site()))
                }
            ))
        };

        // Output
        let output = EnumAttr::from_attributes(&input).unwrap();

        // Test
        assert_eq!(expected_output, output)
    }

    #[test]
    fn enum_attribute_success2() {
        // Define input
        let input_stream = quote! {
            #[pod(other(), other2 = val)]
            struct Foo;
        };

        let input = parse2::<DeriveInput>(input_stream).unwrap().attrs;

        // Define expected output
        let expected_output = EnumAttr {
            repr: None
        };

        // Output
        let output = EnumAttr::from_attributes(&input).unwrap();

        // Test
        assert_eq!(expected_output, output)
    }

    #[test]
    fn enum_attribute_fail() {
        // Define input
        let input_stream = quote! {
            #[pod(repr = u8)]
            struct Foo;
        };

        let input = parse2::<DeriveInput>(input_stream).unwrap().attrs;

        // Define expected output
        let expected_output = Err("repr attribute must be of shape repr($type)");

        // Output
        let output = EnumAttr::from_attributes(&input);

        // Test
        assert_eq!(expected_output, output)
    }

    #[test]
    fn enum_attribute_fail2() {
        // Define input
        let input_stream = quote! {
            #[pod(repr(3))]
            struct Foo;
        };

        let input = parse2::<DeriveInput>(input_stream).unwrap().attrs;

        // Define expected output
        let expected_output = Err("repr attribute must contain a type");

        // Output
        let output = EnumAttr::from_attributes(&input);

        // Test
        assert_eq!(expected_output, output)
    }
}