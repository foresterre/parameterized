use parameterized_macro::parameterized;

// a trailing comma after v and w's arguments (multiple inputs) and after every attribute list
#[parameterized(
    v = { 1, 2, 3, },
    w = { 1, 2, 3, },
)]
#[parameterized_macro(tokio::test)]
fn my_test(v: u32, w: u32) {
    assert!(true);
}

fn main() {}
