extern crate parameterized_macro;

pub use parameterized_macro::parameterized;

#[cfg(test)]
mod transitive_attrs {
    use super::parameterized as pm;

    // for intellij-rust
    #[test]
    fn _mark_module_as_test() {}

    #[pm(input = { None, None, None })]
    #[should_panic]
    fn numbers(input: Option<()>) {
        input.unwrap()
    }
}
