#![allow(warnings)] // TODO: remove

#[macro_use]
extern crate syn;
extern crate proc_macro;

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::spanned::Spanned;

use attribute::AttributeArgList;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::iter::FromIterator;
use syn::token::Token;

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

    // uniqueness test: are all identifiers in the attribute unique?
    let mut set = HashSet::new();
    if !args
        .args
        .iter()
        .all(|list_of_arg| set.insert(list_of_arg.id.clone()))
    {
        panic!("Duplicate identifier found. Please use unique parameter names.")
    }

    // step 1 impl
    let exprs_by_id: HashMap<syn::Ident, Vec<syn::Expr>> = args
        .args
        .iter()
        .map(|v| (v.id.clone(), v.param_args.iter().cloned().collect()))
        .collect();

    // step 2 impl
    let (eq, amount) = equal_amount_of_expr(&exprs_by_id);
    if !eq {
        let exprs_by_id: BTreeMap<syn::Ident, Vec<syn::Expr>> = BTreeMap::from_iter(exprs_by_id);

        let mut ids: String = exprs_by_id
            .iter()
            .map(|(id, _)| format!("{}", id))
            .collect::<Vec<String>>()
            .join(", ");

        panic!("All inputs ({}) should have equal length.", ids)
    }

    // step 3 impl
    if let Some(cases) = amount {
        let test_case_fns = (0..cases).into_iter().map(|i| {
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

        return token_stream.into();
    } else {
        panic!("Unable to construct parameterized test cases.");
    }
}

// fixme: this is not pretty, but at least it is a single pass
// returns whether all lengths are equal, and if any expr exists, the amount (wrapped in an option)
fn equal_amount_of_expr(map: &HashMap<syn::Ident, Vec<syn::Expr>>) -> (bool, Option<usize>) {
    let mut max: Option<usize> = None;

    for (_id, exprs) in map {
        if let Some(current_max) = max {
            if current_max != exprs.len() {
                return (false, max);
            }
        } else {
            max = Some(exprs.len())
        }
    }

    return (true, max);
}
