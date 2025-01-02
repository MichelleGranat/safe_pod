use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{Expr, Variant};

/// Implementation of from_le_bytes expression for unit variants
pub fn from_le_expression(variant: &Variant, match_expr: &Expr) -> TokenStream {
    // Implement from le bytes match expression for unit variant
    let ident = &variant.ident;
    quote_spanned! {variant.ident.span() => 
        #match_expr => { Self::#ident },
    }
}

/// Implementation of from_be_bytes expression for unit variants
pub fn from_be_expression(variant: &Variant, match_expr: &Expr) -> TokenStream {
    // Implement from be bytes match expression for unit variant
    let ident = &variant.ident;
    quote_spanned! {variant.ident.span() => 
        #match_expr => { Self::#ident },
    }
}

/// Implementation of to_le_bytes expression for unit variants
pub fn to_le_expression(variant: &Variant, match_expr: &Expr) -> TokenStream {
    // Implement to le bytes match expression for unit variant
    let ident = &variant.ident;
    quote_spanned! {variant.ident.span() => 
        Self::#ident => { #match_expr },
    }
}

/// Implementation of to_be_bytes expression for unit variants
pub fn to_be_expression(variant: &Variant, match_expr: &Expr) -> TokenStream {
    // Implement to be bytes match expression for unit variant
    let ident = &variant.ident;
    quote_spanned! {variant.ident.span() => 
        Self::#ident => { #match_expr },
    }
}
