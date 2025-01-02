use syn::{Attribute, Expr};

use super::utils;

/// `pod` attribute struct for a variant
#[derive(Debug, PartialEq, Eq)]
pub struct VariantAttr {
    pub match_expr: Option<Expr>
}

impl VariantAttr {
    // Extracts EnumAttr from an attribute vector
    pub fn from_attributes(attributes: &Vec<Attribute>) -> Result<Self, &'static str> {
        // Define fields
        let mut match_expr: Option<Expr> = None;

        // Get inner attributes
        let attrs = match utils::get_pod(attributes) {
            Ok(a) => a,
            Err(e) => return Err(e)
        };

        // Get attributes
        match_expr = match utils::get_match_expr(&attrs) {
            Ok(e) => Some(e),
            Err(e) => match e {
                "not found" => None,
                _ => return Err(e)
            }
        };

        return Ok( VariantAttr { match_expr });
    }
}

#[cfg(test)]
mod tests {
    use syn::{parse2, Data, DeriveInput, ExprLit, Lit};
    use quote::quote;

    use super::*;

    #[test]
    fn variant_attribute_success() {
        // Define input
        let input_stream = quote! {
            enum Foo {
                #[pod(match_expr(32), other(), other2 = val)]
                Bar,
            }
        };

        let input = match parse2::<DeriveInput>(input_stream).unwrap().data {
            Data::Enum(ed) => ed.variants.first().unwrap().attrs.clone(),
            _ => unreachable!()
        };

        // Define expected output
        let expected_output = VariantAttr {
            match_expr: Some(Expr::Lit(ExprLit {
                attrs: Vec::new(),
                lit: parse2::<Lit>(quote! { 32 }).unwrap()
            }))
        };

        // Output
        let output = VariantAttr::from_attributes(&input).unwrap();

        // Test
        assert_eq!(expected_output, output)
    }

    #[test]
    fn variant_attribute_success2() {
        // Define input
        let input_stream = quote! {
            enum Foo {
                #[pod(other(), other2 = val)]
                Bar,
            }
        };

        let input = match parse2::<DeriveInput>(input_stream).unwrap().data {
            Data::Enum(ed) => ed.variants.first().unwrap().attrs.clone(),
            _ => unreachable!()
        };

        // Define expected output
        let expected_output = VariantAttr {
            match_expr: None
        };

        // Output
        let output = VariantAttr::from_attributes(&input).unwrap();

        // Test
        assert_eq!(expected_output, output)
    }

    #[test]
    fn variant_attribute_fail() {
        // Define input
        let input_stream = quote! {
            enum Foo {
                #[pod(match_expr = 32, other(), other2 = val)]
                Bar,
            }
        };

        let input = match parse2::<DeriveInput>(input_stream).unwrap().data {
            Data::Enum(ed) => ed.variants.first().unwrap().attrs.clone(),
            _ => unreachable!()
        };

        // Define expected output
        let expected_output = Err("match attribute must be of shape macth_expr($expression)");

        // Output
        let output = VariantAttr::from_attributes(&input);

        // Test
        assert_eq!(expected_output, output)
    }

    #[test]
    fn variant_attribute_fail2() {
        // Define input
        let input_stream = quote! {
            enum Foo {
                #[pod(match_expr(if), other(), other2 = val)]
                Bar,
            }
        };

        let input = match parse2::<DeriveInput>(input_stream).unwrap().data {
            Data::Enum(ed) => ed.variants.first().unwrap().attrs.clone(),
            _ => unreachable!()
        };

        assert!(!input.is_empty());

        // Define expected output
        let expected_output = Err("match attribute must contain an expression");

        // Output
        let output = VariantAttr::from_attributes(&input);

        // Test
        assert_eq!(expected_output, output)
    }
}