### High level overview of test case generation

#### An example

With this macro we generate the following code (1) from the given parameterized test definition (0):

**parameterized test definition:**

```rust
// 0

#[parameterized(values = {1, 2})]
fn my_test(a: i32) {
    assert!(a > 0)
}
```


**generated test cases:**
```rust
// 1

#[cfg(test)]
mod my_test {
    #[test]
    fn case_0() {
        let a: i32 = 1;
        assert!(a > 0)
    }

    #[test]
    fn case_1() {
        let a: i32 = 2;
        assert!(a > 0)
    }
}
```

More examples can be found in the `expand` crate, and the tests.

#### notes:
- The function name in (1) is the same as the module name in (0)

- Note that arguments are not limited to primitives; they can be any expression (assuming:)

- In a parameterized test case, the input arguments (which are expressions) specified in the attribute should evaluate
  to the same type as their identically named companions in the function signature.

- Tests executed from the workspace crate should be run individually, e.g.
    (`cargo test --package parameterized-macro --test tests individual_cases -- --exact`).
    Otherwise, if just `cargo test` is used, some generated test cases will run in an incorrect context setting.

#### todo's and fixme's:
- see 'fixme' in code comments, tests
- see 'tests/todo' test cases

- propagate other attributes to the generated test cases
- use `heck` crate to fix casing
- the current code base is a first version; the code is not particularly pretty.