use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ItemFn, Lit, LitStr, MetaNameValue, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

/// Arguments for the rule macro.
struct RuleArgs {
    name: LitStr,
    history: usize,
}

/// Helper function to parse a `usize` from an expression
fn parse_history_value(expr: &Expr) -> syn::Result<usize> {
    if let Expr::Lit(expr_lit) = expr {
        if let Lit::Int(lit_int) = &expr_lit.lit {
            return lit_int.base10_parse::<usize>();
        }
    }
    // If it's not an integer literal, returns an error
    Err(syn::Error::new_spanned(expr, "Expected an integer literal for 'history' value"))
}


impl Parse for RuleArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: LitStr = input.parse()?;
        let mut history = 0;

        // Checks for any further arguments after the name
        if !input.is_empty() {
            input.parse::<Token![,]>()?;

            // Parses a comma-separated list of `key = value` attributes
            let attrs = Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?;

            for attr in attrs {
                // Finds the attribute with the name 'history' and parses its value
                if attr.path.is_ident("history") {
                    history = parse_history_value(&attr.value)?;
                }
            }
        }

        Ok(RuleArgs { name, history })
    }
}

#[proc_macro_attribute]
/// A procedural attribute macro to register a Chaos Game rule function.
pub fn rule(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parses the macro inputs
    let function = parse_macro_input!(input as ItemFn);
    let rule_args = parse_macro_input!(args as RuleArgs);
    let rule_name = rule_args.name;
    let rule_history = rule_args.history;

    // Gets the name of the function written by the user
    let fn_name = &function.sig.ident;

    // Generates the registration code for the rule
    let expanded = quote! {
        #function

        ::inventory::submit! {
            crate::rules::Rule {
                name: #rule_name,
                function: #fn_name,
                history: #rule_history,
            }
        }
    };

    TokenStream::from(expanded)
}