#[macro_use]
extern crate syn;
extern crate proc_macro;

use quote::quote;
use syn::{parse_macro_input, Block, Expr, Ident, Type, Visibility};

mod attribute;
mod fun;

#[proc_macro_attribute]
pub fn parameterized(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let tests = parse_macro_input!(args as attribute::TestCases);
    let fun = parse_macro_input!(input as fun::TestFn);

    let name = fun.name();
    let vis = fun.vis();
    let mod_name = format!("{}", name);
    let mod_ident = syn::Ident::new(mod_name.as_str(), name.span());

    let generated_cases = tests.cases().iter().map(|case| {
        generate_test_case(
            case.identifier(),
            &fun.parameters(),
            &case.inputs(),
            fun.body(),
            vis,
            fun.attrs().as_slice(),
        )
    }).collect::<Vec<_>>();

    let token_stream = quote! {
        #[cfg(test)]
        #vis mod #mod_ident {
            use super::*;

            #(#generated_cases)*
        }
    };

    token_stream.into()
}

fn generate_test_case(
    test_name: &syn::Ident,
    params: &[(&Ident, &Type)],
    exprs: &[&Expr],
    body: &Block,
    vis: &Visibility,
    attributes: &[syn::Attribute],
) -> proc_macro2::TokenStream {
    let bindings = (0..params.len()).map(|i| create_binding(params[i], exprs[i]));

    quote! {
        #[test]
        #(#attributes)*
        #vis fn #test_name() {
            #(#bindings)*
            #body
        }
    }
}

fn create_binding(param: (&Ident, &Type), expr: &Expr) -> proc_macro2::TokenStream {
    let (ident, typ) = param;

    quote! {
        let #ident: #typ = #expr;
    }
}
