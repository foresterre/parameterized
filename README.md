# parameterized

Procedural macro which allows you to define a test to be run with multiple (optionally different) arguments.
Test cases are defined using the 'parameterized' attribute instead of the 'test' attribute.
This crate was inspired by JUnit `@ParameterizedTest`.

### Examples:

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

Additional examples can be found <a href="https://github.com/foresterre/parameterized-examples">here</a> (parameterized-examples repo)
and in the <a href="macro/tests">tests</a> folder.


<br>

### Imports

If you prefer not to import this library with `use parameterized::parameterized;` in every test case, you can put
the following snippet at the top of your crate root:
```rust
#[cfg(test)]
#[macro_use]
extern crate parameterized;
```

### IDE 'run test' intent

IntelliJ IDEA recognizes test cases and provides context menu's which allow you to run tests within a certain scope
(such as a module or a single test case). For example, in IntelliJ you can usually run individual test cases by clicking
the ▶ icon in the gutter. Unfortunately, attribute macros are currently not expanded by `intellij-rust`.
This means that the IDE will not recognize test cases generated as a result of attribute macros (such as the
`parameterized` macro published by this crate). As a partial work around, you can create an empty test case which
will mark the module as containing test cases and in the gutter you will find a ▶ icon next to the module. This allows
you to run test cases per module. This crate provides a macro called `ide!()` which creates an empty test case for
the above purpose.

Note: `intellij-rust` does expand declarative macro's (at least with the new macro engine which can be
selected in the 'settings' menu), such as this `ide!` macro.

```rust
#[cfg(test)]
mod tests {
    use parameterized::parameterized as pm;
    use parameterized::ide;

    ide!();

    #[pm(input = { 2, 3, 4 }, output = { 4, 6, 8 })]
    fn test_add2(input: i32, output: i32) {
        let add2 = |receiver: i32| { receiver + 2 };

        assert_eq(add2(input), output);
    }
}
```

### License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

<br>

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
