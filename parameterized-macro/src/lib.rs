#[macro_use]
extern crate syn;
extern crate proc_macro;

#[cfg(all(feature = "valuesource", not(feature = "matrix")))]
mod value_source;

#[cfg(all(feature = "valuesource", not(feature = "matrix")))]
#[proc_macro_attribute]
pub fn parameterized(
    args: ::proc_macro::TokenStream,
    input: ::proc_macro::TokenStream,
) -> ::proc_macro::TokenStream {
    let argument_lists = parse_macro_input!(args as value_source::AttributeArgList);
    let func = parse_macro_input!(input as ::syn::ItemFn);

    value_source::restructure::impl_value_source(argument_lists, func)
}

#[cfg(all(feature = "matrix", not(feature = "valuesource")))]
mod case_by_case;

#[cfg(all(feature = "matrix", not(feature = "valuesource")))]
#[proc_macro_attribute]
pub fn parameterized(
    args: ::proc_macro::TokenStream,
    input: ::proc_macro::TokenStream,
) -> ::proc_macro::TokenStream {
    let tests = parse_macro_input!(args as case_by_case::test_cases::TestCases);
    let fun = parse_macro_input!(input as case_by_case::fun::TestFn);

    case_by_case::restructure::impl_case_by_case(tests, fun)
}
