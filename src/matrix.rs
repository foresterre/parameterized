#[cfg(feature = "matrix")]
#[cfg(not(feature = "valuesource"))]
#[cfg(test)]
mod tests {
    use crate::parameterized as pm;

    fn add5<T: Into<u32>>(component: T) -> u32 {
        component.into() + 5
    }

    mod readme_test {
        use super::*;

        ide!();

        #[pm(zero = {
            0, 5
        }, one = {
            1, 6
        }, two = {
            2, 7
        })]
        fn test_add5(eh: u16, expected: u32) {
            assert_eq!(add5(eh), expected)
        }
    }

    mod marked_as_test_module {
        use super::*;

        ide!();

        #[pm(two = { 2, 4 }, six = { 6, 12 }, eight = { 8, 16 })]
        fn test_times2(input: i32, output: i32) {
            let times2 = |receiver: i32| receiver * 2;

            assert_eq!(times2(input), output);
        }
    }

    mod transitive_attrs {
        use super::*;

        ide!();

        #[pm(none = { None })]
        #[should_panic]
        fn numbers(input: Option<()>) {
            input.unwrap()
        }
    }
}
