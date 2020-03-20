#[macro_use]
extern crate syn;
extern crate proc_macro;

#[proc_macro_attribute]
pub fn case(
    _args: proc_macro::TokenStream,
    _input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    panic!("This is a template; the parameterized_case macro will soon be implemented.");
}
