#[cfg(all(feature = "valuesource", not(feature = "matrix")))]
#[cfg(test)]
mod _tests {
    enum Fruit {
        Apple,
        Pear,
        Banana,
        Bramble(BrambleFruit),
    }

    trait NameOf {
        fn name_of(&self) -> &str;
    }

    impl NameOf for Fruit {
        fn name_of(&self) -> &str {
            match self {
                Fruit::Apple => "apple",
                Fruit::Pear => "pear",
                Fruit::Banana => "banana",
                Fruit::Bramble(fruit) => fruit.name_of(),
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

    #[parameterized(fruit = {
        Fruit::Apple, Fruit::Pear, Fruit::Banana, Fruit::Bramble(BrambleFruit::Blackberry)
    }, name = {
        "apple", "pear", "banana", "blackberry"
    })]
    fn a_fruity_test(fruit: Fruit, name: &str) {
        assert_eq!(fruit.name_of(), name)
    }
}

fn main() {
    println!("examples/fruits.rs: Copy the contents of this file to a place where you can conveniently run `cargo test` and run it to see the output.")
}
