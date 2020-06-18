use parameterized_macro::parameterized;

#[parameterized(zzz = { "a", "b" }, aaa = { 1, 2, 3 })]
pub(crate) fn my_test(v: &str, w: i32) {}

// fixme: The current error message orders based on Indent cmp order in a BTreeMap.
//        In the above case, it will report (aaa, zzz)
//        Instead we would like to order based on the order in the attribute, i.e. (zzz, aaa)
//
// One way to solve this is to replace the current HashMap with a Vec<(Ident, Vec<Expr>)>
fn main() {}
