use syn::{punctuated::Punctuated, Attribute, Expr, Meta, Token, Type};

// Utility function to parse `#[pod(..)]` attribute and return inner attributes
pub fn get_pod(attributes: &Vec<Attribute>) -> Result<Punctuated<Meta, Token![,]>, &'static str> {
    // Go over attribute list
    for attribute in attributes {
        // If attribute is `pod`
        if attribute.meta.path().is_ident("pod") {
            // Get contents
            let contents = match attribute.meta.require_list() {
                Ok(c) => c,
                Err(_) => return Err("pod attribute must be of shape #[pod(...)]")
            };

            // Parse content into attributes
            let attrs = match contents.parse_args_with(
                Punctuated::<Meta, Token![,]>::parse_terminated
            ) {
                Ok(a) => a,
                Err(_) => return Err("pod attribute values must be a comma separated list")
            };

            // Return inner attributes
            return Ok(attrs);
        }
    }

    // Else return not found
    return Err("not found");
}

// Utility function to parse `repr(type)` attribute
pub fn get_repr(attributes: &Punctuated<Meta, Token![,]>) -> Result<Type, &'static str> {
    // Go over attributes
    for attribute in attributes {
        // If attribute is `repr`
        if attribute.path().is_ident("repr") {
            // Get contents
            let content = match attribute.require_list() {
                Ok(l) => l,
                Err(_) => return Err("repr attribute must be of shape repr($type)")
            };

            // Get type
            let ty = match content.parse_args::<Type>() {
                Ok(t) => t,
                Err(_) => return Err("repr attribute must contain a type")
            };

            // Return type
            return Ok(ty)
        }
    }

    return Err("not found");
}

// Utility function to parse `match_expr(expr)` attribute
pub fn get_match_expr(attributes: &Punctuated<Meta, Token![,]>) -> Result<Expr, &'static str> {
    // Go over attributes
    for attribute in attributes {
        // If attribute is `repr`
        if attribute.path().is_ident("match_expr") {
            // Get contents
            let content = match attribute.require_list() {
                Ok(l) => l,
                Err(_) => return Err("match attribute must be of shape macth_expr($expression)")
            };

            // Get type
            let expr = match content.parse_args::<Expr>() {
                Ok(e) => e,
                Err(_) => return Err("match attribute must contain an expression")
            };

            // Return type
            return Ok(expr)
        }
    }

    return Err("not found");
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use syn::{parse2, DeriveInput, Ident, Path, TypePath, Data};
    use quote::quote;

    use super::*;

    #[test]
    fn get_pod_success() {
        // Define input
        let input_stream = quote! {
            #[pod(repr(u16))]
            struct Foo;
        };

        let input = parse2::<DeriveInput>(input_stream).unwrap().attrs;

        // Output
        let output = get_pod(&input);

        // Test
        assert!(output.is_ok())
    }

    #[test]
    fn get_pod_fail() {
        // Define input
        let input_stream = quote! {
            #[pod = repr(u16)]
            struct Foo;
        };

        let input = parse2::<DeriveInput>(input_stream).unwrap().attrs;

        // Define expected output
        let expected_output = Err("pod attribute must be of shape #[pod(...)]");

        // Output
        let output = get_pod(&input);

        // Test
        assert_eq!(expected_output, output)
    }

    #[test]
    fn get_pod_fail2() {
        // Define input
        let input_stream = quote! {
            #[pod(repr(u16) | hello)]
            struct Foo;
        };

        let input = parse2::<DeriveInput>(input_stream).unwrap().attrs;

        // Define expected output
        let expected_output = Err("pod attribute values must be a comma separated list");

        // Output
        let output = get_pod(&input);

        // Test
        assert_eq!(expected_output, output)
    }

    #[test]
    fn get_repr_success() {
        // Define input
        let input_stream = quote! {
            #[pod(repr(u16))]
            struct Foo;
        };

        let input = parse2::<DeriveInput>(input_stream).unwrap().attrs;
        let meta_input = get_pod(&input).unwrap();

        // Output
        let output = get_repr(&meta_input);

        // Test
        assert!(output.is_ok())
    }

    #[test]
    fn get_repr_fail() {
        // Define input
        let input_stream = quote! {
            #[pod(repr = u16)]
            struct Foo;
        };

        let input = parse2::<DeriveInput>(input_stream).unwrap().attrs;
        let meta_input = get_pod(&input).unwrap();

        // Define expected output
        let expected_output = Err("repr attribute must be of shape repr($type)");

        // Output
        let output = get_repr(&meta_input);

        // Test
        assert_eq!(expected_output, output)
    }

    #[test]
    fn get_repr_fail2() {
        // Define input
        let input_stream = quote! {
            #[pod(repr(12))]
            struct Foo;
        };

        let input = parse2::<DeriveInput>(input_stream).unwrap().attrs;
        let meta_input = get_pod(&input).unwrap();

        // Define expected output
        let expected_output = Err("repr attribute must contain a type");

        // Output
        let output = get_repr(&meta_input);

        // Test
        assert_eq!(expected_output, output)
    }

    #[test]
    fn get_match_expr_success() {
        // Define input
        let input_stream = quote! {
            #[pod(repr(u16))]
            enum Foo {
                #[pod(match_expr(0))]
                Bar,
            }
        };

        let input = match parse2::<DeriveInput>(input_stream).unwrap().data {
            Data::Enum(ed) => ed.variants.first().unwrap().attrs.clone(),
            _ => unreachable!()
        };
        let meta_input = get_pod(&input).unwrap();

        // Output
        let output = get_match_expr(&meta_input);

        // Test
        assert!(output.is_ok());
    }

    #[test]
    fn get_match_expr_fail() {
        // Define input
        let input_stream = quote! {
            #[pod(repr(u16))]
            enum Foo {
                #[pod(match_expr = 0)]
                Bar,
            }
        };

        let input = match parse2::<DeriveInput>(input_stream).unwrap().data {
            Data::Enum(ed) => ed.variants.first().unwrap().attrs.clone(),
            _ => unreachable!()
        };
        let meta_input = get_pod(&input).unwrap();

        // Define expected output
        let expected_output = Err("match attribute must be of shape macth_expr($expression)");

        // Output
        let output = get_match_expr(&meta_input);

        // Test
        assert_eq!(expected_output, output)
    }

    #[test]
    fn get_match_expr_fail2() {
        // Define input
        let input_stream = quote! {
            #[pod(repr(u16))]
            enum Foo {
                #[pod(match_expr(if))]
                Bar,
            }
        };

        let input = match parse2::<DeriveInput>(input_stream).unwrap().data {
            Data::Enum(ed) => ed.variants.first().unwrap().attrs.clone(),
            _ => unreachable!()
        };
        let meta_input = get_pod(&input).unwrap();

        // Define expected output
        let expected_output = Err("match attribute must contain an expression");

        // Output
        let output = get_match_expr(&meta_input);

        // Test
        assert_eq!(expected_output, output)
    }

}