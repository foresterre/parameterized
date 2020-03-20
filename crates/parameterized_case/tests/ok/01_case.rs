use parameterized_case::case;

// case(<name>, parameters...)
#[case(one, v = 1)]
#[case(two, v = 2)]
#[case(three, v = 3)]
fn numbers(v: i32) {
    assert!(v > 0);
}

fn main() {}
