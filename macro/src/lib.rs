#[macro_use]
extern crate syn;
extern crate proc_macro;

use std::collections::{BTreeMap, HashMap};
use std::iter::FromIterator;
use std::sync::atomic::{AtomicUsize, Ordering};

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
    let args = parse_macro_input!(args as attribute::AttributeArgList);
    let func = parse_macro_input!(input as syn::ItemFn);

    let name = &func.sig.ident;
    let vis = &func.vis;
    let func_args = &func.sig.inputs;
    let body_block = &func.block;

    let mod_name = format!("{}", name);
    let mod_ident = syn::Ident::new(mod_name.as_str(), name.span());

    // Used to give generated test cases unique names.
    let generated_ident_id = AtomicUsize::new(0);

    // **implementation idea**
    //
    // step 1
    //
    // map {
    //   v -> ...EXPR*_v,
    //   w -> ...EXPR*_w,
    // }
    // .collect<Ident, Vec<Expr>>   // len(v) ?= len(w) ?= ...IDENT*
    //
    // step 2
    //
    // check that all EXPR* have the same length, else err
    //
    // step 3
    //
    // now we need to create test cases, one for each EXPR, consisting of:
    // - `let #ident: #ty = #expr;` bind at the start of the fn, #ident is key of the map,
    //      #ty is the matching fn param, #expr is the current expr
    // - then append the body (block) of the fn
    //

    let identifiers_defined = args.args.len();

    // step 1 impl
    let exprs_by_id: HashMap<syn::Ident, Vec<syn::Expr>> = args
        .args
        .iter()
        .map(|v| (v.id.clone(), v.param_args.iter().cloned().collect()))
        .collect();

    // interlude: ensure that the parameterized test definition contain unique identifiers.
    if exprs_by_id.len() != identifiers_defined {
        panic!("Duplicate identifier(s) found. Please use unique parameter names.")
    }

    // step 2 impl
    let amount_of_test_cases = check_all_input_lengths(&exprs_by_id);

    // step 3 impl
    let test_case_fns = (0..amount_of_test_cases).map(|i| {
        let binds: Vec<TokenStream> = func_args
            .iter()
            .map(|fn_arg| {
                // we require an argument (name: Type) to be Typed ,
                // and not Receiver (a variant of self).
                if let syn::FnArg::Typed(pat) = fn_arg {
                    let fn_expected_ty = &pat.ty;
                    let fn_ident = pat.pat.as_ref();

                    // The following is a dance to obtain the actual identifier.
                    if let syn::Pat::Ident(pat_ident) = fn_ident {
                        let fn_arg_ident = &pat_ident.ident;

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
                        if let Some(exprs) = exprs_by_id.get(&fn_arg_ident) {
                            let expr = &exprs[i];

                            // A let binding is constructed so we can type check the given expression.
                            return quote! {
                                let #fn_arg_ident: #fn_expected_ty = #expr;
                            };
                        } else {
                            // This should not be possible, since we check use as range exactly
                            // the amount of cases and check that the input argument lists are
                            // equal to one another.
                            panic!("not enough test cases found, [this should never happen] ")
                        }
                    } else {
                        // This should also never happen. But perhaps it could, I'm not sure.
                        panic!("Unable to find a parameter name...")
                    }
                } else {
                    // Idem, not sure whether this can even happen either.
                    panic!("Malformed function input.")
                }
            })
            .collect(); // end of construction of let bindings

        let next_id = generated_ident_id.fetch_add(1, Ordering::SeqCst);
        let ident = format!("case_{}", next_id);
        let ident = syn::Ident::new(ident.as_str(), func.span()); // fixme: span

        quote! {
            #[test]
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
fn check_all_input_lengths(map: &HashMap<syn::Ident, Vec<syn::Expr>>) -> usize {
    map.values()
        .fold(None, |acc: Option<usize>, exprs| match acc {
            Some(size) if size == exprs.len() => Some(size),
            Some(_) => {
                panic_on_inequal_length(map);
                unreachable!()
            }
            None => Some(exprs.len()),
        })
        .unwrap_or_default()
}

/// When this function gets invoked, it will construct an error message and then panic! with that message.
fn panic_on_inequal_length<K: Ord + Display, V>(map: impl IntoIterator<Item = (K, V)>) {
    let sorted_by_id: BTreeMap<K, V> = BTreeMap::from_iter(map);

    let ids: String = sorted_by_id
        .iter()
        .map(|(id, _)| format!("{}", id))
        .collect::<Vec<String>>()
        .join(", ");

    panic!("All inputs ({}) should have equal length.", ids)
}
