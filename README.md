# parameterized

Procedural macro which allows you to define a test to be run with multiple (optionally different) arguments.
Test cases are defined using the 'parameterized' attribute instead of the 'test' attribute.
This crate was inspired by JUnit `@ParameterizedTest`.

### Examples:

Additional examples can be found at the <a href="https://github.com/foresterre/parameterized-examples">parameterized-examples repository</a>,
the <a href="https://github.com/foresterre/parameterized-example-usage">example-usage</a> crate in this repository and in the <a href="parameterized-macro/tests">tests</a> folder.

<br>

**Example: Add5**

```rust
fn add5<T: Into<u32>>(component: T) -> u32 {
    component.into() + 5
}

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized;

    ide!();

    #[parameterized(input = {
        0, 1, 2
    }, expected = {
        5, 6, 7
    })]
    fn test_add5(input: u16, expected: u32) {
        assert_eq!(add5(input), expected);
    }
}
```

**Example: Fruits**

```rust
enum Fruit {
    Apple,
    Bramble(BrambleFruit),
    Pear,
}

trait NameOf {
    fn name_of(&self) -> &str;
}

impl NameOf for Fruit {
    fn name_of(&self) -> &str {
        match self {
            Fruit::Apple => "apple",
            Fruit::Bramble(fruit) => fruit.name_of(),
            Fruit::Pear => "pear",
        }
    }
}

enum BrambleFruit {
    Blackberry,
}

impl NameOf for BrambleFruit {
    fn name_of(&self) -> &str {
        match self {
            BrambleFruit::Blackberry => "blackberry",
        }
    }
}

#[cfg(test)]
mod tests {
    use parameterized::parameterized;


    #[parameterized(fruit = {
        Fruit::Apple, Fruit::Pear, Fruit::Bramble(BrambleFruit::Blackberry)
    }, name = {
        "apple", "pear", "blackberry"
    })]
    fn a_fruity_test(fruit: Fruit, name: &str) {
        assert_eq!(fruit.name_of(), name)
    }
}
```

<br>

### Imports

If you prefer not to import this library (with `use parameterized::parameterized;`) in every test module, you can put
the following snippet at the top of your crate root:
```rust
#[cfg(test)]
#[macro_use]
extern crate parameterized;
```

### Modes

This library consists of two modes: `valuesource` (the default mode, which is shown above) and `casebycase`. 
The primary difference between these two modes is the way in which values are provided to the
`parameterized` macro. In case of `valuesource`, all values of a each parameter are given in as sequence, and each
sequence must have the same length, as each i-th element defines the inputs for a test case. A short example using
the syntax of this mode can be found below. Note here that the name of the formal parameter within the parameterized macro
should be equal to the name of the function it is defined on (`test`).

```rust

// generates the following test cases:
// * `case_1(1, 'a')`
// * `case_2(2, 'b')`
#[parameterized(ints = {1,2}, chars = {'a', 'b'})]
fn test(ints: i32, chars: char) {
    assert!(...)
}
```

The second mode, `casebycase` can be used to define test cases as a sequence of input values
for a single test case at a time. The example below is semantically equivalent to the above example.
The syntax however differs; and note that the formal parameters now are no longer required to
be the same within `parameterized` and the function. Instead, the identifiers for a sequence of values (a test case)
within `parameterized` represent the name of the test case. If we would rename `case_1` to `one_and_a` and
`case_2` to `two_and_b`, the identifiers of the generated test cases would change accordingly.
This mode is especially useful with lots of test cases.
 
```rust
// generates the following test cases:
// * `case_1(1, 'a')`
// * `case_2(2, 'b')`
#[parameterized(case_1 = {1, 'a'}, case_2 = {'2', 'b'})]
fn test(ints: i32, chars: char) {
    assert!(...)
}
```

A mode can be selected by activating their name as a Rust feature. By default, `valuesource` is used. If you want to use
`casebycase` instead, you should set `default-features` to `false` and add the `casebycase` feature. For example:
```toml
[dev-dependencies]
parameterized = { version = "0.3", default-features = false, features = ["casebycase"] }
```

Since Rust features are used, only one mode can be used per crate.

### IDE 'run test' intent

IntelliJ IDEA recognizes test cases and provides context menus which allow you to run tests within a certain scope
(such as a module or a single test case). For example, in IntelliJ you can usually run individual test cases by clicking
the ▶ icon in the gutter. Unfortunately, attribute macros are currently not expanded by `intellij-rust`.
This means that the IDE will not recognize test cases generated as a result of attribute macros (such as the
`parameterized` macro published by this crate). 

A workaround can be found below (if you have a better solution, please feel free to open an issue; thank you in advance!)

```rust
fn squared(input: i8) -> i8 {
  input * input  
}

#[cfg(test)]
mod tests {
    use super::*;

    use parameterized::parameterized as pm;
    use parameterized::ide;
        
    mod squared_tests { // <--
        use super::*;

        ide!(); // <--
    
        #[pm(input = {
            -2, -1, 0, 1, 2
        }, expected = {
            4, 1, 0, 1, 4
        })]
        fn test_squared(input: i8, output: i8) {
            assert_eq(squared(input), output);
        }
    }
}
```

Here we created an empty test case (using the `ide!()` macro) which will mark the surrounding module as 'containing test cases'. In
the gutter you will find the ▶ icon next to the module. This allows you to run test cases per module.

Note: `intellij-rust` does expand declarative macro's (with the new macro engine which can be
selected in the 'settings' menu), such as this `ide!` macro.


### License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

<br>

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
