fn main() {}

enum A {
    One,
    Two,
    Three,
}

impl A {
    fn yeah(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod le_spekje {
    use super::*;
    use parameterized::test_case;

    test_case!(yellow, (p: i32, q: A),
    [
        p = {1, 2, 3},
        q = {A::One, A::Two, A::Three}
    ], {
        assert!(p < 4 && q.yeah());
    });

    test_case!(pink, (p: i32, q: A),
    [
        p = {1, 2, 3},
        q = {A::One, A::Two, A::Three}
    ], {
        assert!(p < 4 && q.yeah());
    });
}
