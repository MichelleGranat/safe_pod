use syn::{punctuated::Punctuated, Attribute, Expr, Meta, Token};

use super::utils;

/// `pod` attribute struct for a variant
pub struct VariantAttr {
    match_expr: Option<Expr>
}

impl VariantAttr {
    // Extracts EnumAttr from an attribute vector
    pub fn from_attributes(attributes: &Vec<Attribute>) -> Result<Self, &'static str> {
        // Define fields
        let mut match_expr: Option<Expr> = None;

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

        // Else return not found
        return Err("not found");
    }
}