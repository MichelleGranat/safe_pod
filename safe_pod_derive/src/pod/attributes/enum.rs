use syn::{punctuated::Punctuated, Attribute, Meta, Type, Token};

use super::utils;

/// `pod` attribute struct for an enum
pub struct EnumAttr {
    repr: Option<Type>
}

impl EnumAttr {
    // Extracts EnumAttr from an attribute vector
    pub fn from_attributes(attributes: &Vec<Attribute>) -> Result<Self, &'static str> {
        // Define fields
        let mut repr: Option<Type> = None;

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

        // Else return not found
        return Err("not found");
    }
}