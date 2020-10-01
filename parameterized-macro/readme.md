### High level overview of test case generation

#### An example

With this macro we generate the following code (1) from the given parameterized test definition (0):

**parameterized test definition:**

```rust
// 0

#[parameterized(first = {1, "wanderlust"}, second = { 2, "wanderer" })]
fn my_test(a: i32, b: &str) {
    assert!(a > 0 && b.starts_with("w"))
}
```


**generated test cases:**
```rust
// 1

#[cfg(test)]
mod my_test {
    #[test]
    fn first() {
        let a: i32 = 1;
        let b: &str = "wanderlust";
        assert!(a > 0 && b.starts_with("w"))
    }

    #[test]
    fn second() {
        let a: i32 = 2;
        let b: &str = "wanderer";
        assert!(a > 0 && b.starts_with("w"))
    }
}
```

More examples can be found in the `expand` crate, and the tests.
