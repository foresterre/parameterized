use indexmap::IndexMap;
use proc_macro2::{TokenStream, Span};
use quote::quote;

pub(crate) fn impl_value_source(
    argument_lists: super::AttributeArgList,
    func: syn::ItemFn,
) -> proc_macro::TokenStream {
    let name = &func.sig.ident;
    let vis = &func.vis;
    let func_args = &func.sig.inputs;
    let body_block = &func.block;
    let attributes = &func.attrs;

    let mod_name = format!("{}", name);
    let mod_ident = syn::Ident::new(mod_name.as_str(), name.span());

    // For each provided argument (per parameter), we create a let bind at the start of the fn:
    // * `let #ident: #ty = #expr;`
    // After that we append the body of the test function
    let identifiers_len = argument_lists.args.len();

    let values = argument_lists
        .args
        .iter()
        .map(|v| {
            (
                &v.id,
                v.param_args.iter().collect::<Vec<&syn::Expr>>(),
            )
        })
        .collect::<IndexMap<&syn::Ident, Vec<&syn::Expr>>>();

    // interlude: ensure that the parameterized test definition contain unique identifiers.
    if values.len() != identifiers_len {
        panic!("[parameterized-macro] error: Duplicate identifier(s) found. Please use unique parameter names.")
    }

    let amount_of_test_cases = super::validation::check_all_input_lengths(&values);

    let test_case_fns = (0..amount_of_test_cases).map(|i| {
        let binds: Vec<TokenStream> = func_args
            .iter()
            .map(|fn_arg| {
                if let syn::FnArg::Typed(syn::PatType { pat, ty, .. }) = fn_arg {
                    if let syn::Pat::Ident(syn::PatIdent { ident, .. }) = pat.as_ref() {
                        // Now we use to identifier from the function signature to get the
                        // current (i) test case we are creating.
                        //
                        // If we have `#[parameterized(chars = { 'a', 'b' }, ints = { 1, 2 }]
                        // and the function signature is `fn my_test(chars: char, ints: i8) -> ()`
                        //
                        // then we will two test cases.
                        //
                        // The first test case will substitute (for your mental image,
                        // because in reality it will create let bindings at the start of the
                        // generated test function) the first expressions from the identified
                        // argument lists, in this case from `chars`, `a` and from `ints`, `1`.
                        // The second test case does the same
                        if let Some(exprs) = values.get(ident) {
                            let expr = &exprs[i];

                            // A let binding is constructed so we can type check the given expression.
                            return quote! {
                                let #ident: #ty = #expr;
                            };
                        } else {
                            panic!("[parameterized-macro] error: No matching values found for '{}'", ident);
                        }
                    } else {
                        panic!("[parameterized-macro] error: Function parameter identifier was not found");
                    }
                } else {
                    panic!("[parameterized-macro] error: Given function argument should be typed");
                }
            })
            .collect(); // end of construction of let bindings

        let ident = format!("case_{}", i);
        let ident = syn::Ident::new(ident.as_str(), Span::call_site());

        quote! {
            #[test]
            #(#attributes)*
            #vis fn #ident() {
                #(#binds)*

                #body_block
            }
        }
    });

    // we need to include `use super::*` since we put the test cases in a new module
    let token_stream = quote! {
        #[cfg(test)]
        #vis mod #mod_ident {
            use super::*;

            #(#test_case_fns)*
        }
    };

    token_stream.into()
}
