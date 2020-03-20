use parameterized_case::case;

// case(<name>, parameters...)
#[case(one, v = 1, w = 0)]
#[case(two, v = 2, w = 0)]
#[case(three, v = 3, w = 0)]
fn numbers(v: i32, w: u32) {
    assert!(v > 0 && w == 0);
}

fn main() {}
