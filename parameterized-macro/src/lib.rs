#[macro_use]
extern crate syn;
extern crate proc_macro;

use ordnung::Map;
use proc_macro2::TokenStream;
use quote::quote;
use syn::export::fmt::Display;
use syn::parse_macro_input;
use syn::spanned::Spanned;

mod attribute;

#[proc_macro_attribute]
pub fn parameterized(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // v := { ...exprs },
    // exprs := { e1, e2, e3, ... }
    // e := EXPR
    let argument_lists = parse_macro_input!(args as attribute::AttributeArgList);
    let func = parse_macro_input!(input as syn::ItemFn);

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
                v.id.clone(),
                v.param_args.iter().cloned().collect::<Vec<syn::Expr>>(),
            )
        })
        .collect::<Map<syn::Ident, Vec<syn::Expr>>>();

    // interlude: ensure that the parameterized test definition contain unique identifiers.
    if values.len() != identifiers_len {
        panic!("[parameterized-macro] error: Duplicate identifier(s) found. Please use unique parameter names.")
    }

    let amount_of_test_cases = check_all_input_lengths(&values);

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
        let ident = syn::Ident::new(ident.as_str(), func.span()); // fixme: span

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

/// Checks whether all inputs have equal length.
///
/// All inputs should have equal lengths. Take for example the following example parameterized definition:
/// `#[parameterized(v = { "a", "b", "c" }, w = { 1, 2 })]`
/// Here the length of `v` is 3, while the length of `w` is 2.
/// Since within individual constructed test cases, for all identifiers, values are matched one-by-one
/// the first test shall define `"a"` and `1`, the second `"b"` and 2, but for the third case,
/// a value for `v` exists (namely `"c"`), however no value to substitute for `w` exists.
/// Therefore, no fully valid set of tests can be constructed from the parameterized definition.
fn check_all_input_lengths(map: &Map<syn::Ident, Vec<syn::Expr>>) -> usize {
    let mut arguments: Option<usize> = None;
    for (ident, values) in map.iter() {
        match arguments {
            Some(len) if len == values.len() => continue,
            None => arguments = Some(values.len()),
            _ => panic_on_inequal_length(map.iter(), ident, arguments.unwrap_or_default()),
        }
    }

    arguments.unwrap_or_default()
}

/// When this function gets invoked, it will construct an error message and then panic! with that message.
fn panic_on_inequal_length<K: Ord + Display, V, D: Display>(
    map: impl Iterator<Item = (K, V)>,
    ident: D,
    expected_length: usize,
) {
    let ids: String = map
        .map(|(id, _)| format!("{}", id))
        .collect::<Vec<String>>()
        .join(", ");

    panic!(
        "[parameterized-macro] error: Inconsistent argument list length for '{}'; all inputs ({}) should have equal length (expected = {}).",
        ident,
        ids,
        expected_length
    )
}
