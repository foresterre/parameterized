extern crate parameterized_macro;

pub use parameterized_macro::parameterized;

/// Attribute macro's such as 'parameterized' do not enable the run tests intent for a module
/// marked as cfg(test) (or a #[test] function for that matter) in Intellij.
///
/// To enable the intent within a module, we need at least a single test marked with `#[test]`.
/// The `ide!()` macro is a work around for this issue and creates this empty test. It can be called
/// within every module where we wish to run test cases using the run configuration / run test context
/// menu.
///
/// Using the intellij-rust new macro expansion engine, if this macro is called within a module,
/// the module will be marked as test, and the 'run as test' context menu will be provided in the
/// gutter.
#[macro_export]
macro_rules! ide {
    () => {
        #[test]
        fn __() {}
    };
}

// proto 2

#[macro_export]
macro_rules! test_case {
    (
        $name:ident,
        ( $( $id:ident : $ty:ty ),* ),
        // values provided for `id_declared` should be the same as `id`, otherwise the parameterized macro will panic
        [ $( $id_declared:ident = { $($inputs:expr),*  }),* ],
        { $($body:tt)* }
    ) => {
        #[$crate::parameterized(
            $($id_declared = {$($inputs),*}),*
        )]
        #[test]
        fn $name( $( $id: $ty ),* )  {
            $($body)*
        }
    };
}

#[cfg(test)]
mod test_cases {
    use crate::test_case;

    test_case!(my_test, (p: i32, q: i32, r: u32), {
        assert!(p == q);
    });
}

#[cfg(test)]
mod tests {
    use super::parameterized as pm;

    fn add5<T: Into<u32>>(component: T) -> u32 {
        component.into() + 5
    }

    #[pm(input = {
        0, 1, 2
    }, expected = {
        5, 6, 7
    })]
    fn _test_case(input: u16, expected: u32) {
        assert_eq!(add5(input), expected)
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
