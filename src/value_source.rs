#[cfg(feature = "valuesource")]
#[cfg(not(feature = "casebycase"))]
#[cfg(test)]
mod tests {
    use crate::ide;
    use crate::parameterized as pm;

    fn add5<T: Into<u32>>(component: T) -> u32 {
        component.into() + 5
    }

    mod readme_test {
        use super::*;

        ide!();

        #[pm(input = {
            0, 1, 2
        }, expected = {
            5, 6, 7
        })]
        fn test_add5(input: u16, expected: u32) {
            assert_eq!(add5(input), expected)
        }
    }

    mod marked_as_test_module {
        use super::*;

        ide!();

        #[pm(input = { 2, 3, 4 }, output = { 4, 6, 8 })]
        fn test_times2(input: i32, output: i32) {
            let times2 = |receiver: i32| receiver * 2;

            assert_eq!(times2(input), output);
        }
    }

    mod transitive_attrs {
        use super::*;

        ide!();

        #[pm(input = { None, None, None })]
        #[should_panic]
        fn numbers(input: Option<()>) {
            input.unwrap()
        }
    }
}
