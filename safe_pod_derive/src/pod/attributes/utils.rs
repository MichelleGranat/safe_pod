use syn::{punctuated::Punctuated, Expr, Meta, Token, Type};

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

// Utility function to parse `match(expr)` attribute
pub fn get_match_expr(attributes: &Punctuated<Meta, Token![,]>) -> Result<Expr, &'static str> {
    // Go over attributes
    for attribute in attributes {
        // If attribute is `repr`
        if attribute.path().is_ident("match") {
            // Get contents
            let content = match attribute.require_list() {
                Ok(l) => l,
                Err(_) => return Err("match attribute must be of shape macth($expression)")
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