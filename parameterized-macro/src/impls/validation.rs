#![cfg(not(matrix))]

use indexmap::IndexMap;
use std::fmt::Display;

/// Checks whether all inputs have equal length.
///
/// All inputs should have equal lengths. Take for example the following example parameterized definition:
/// `#[parameterized(v = { "a", "b", "c" }, w = { 1, 2 })]`
/// Here the length of `v` is 3, while the length of `w` is 2.
/// Since within individual constructed test cases, for all identifiers, values are matched one-by-one
/// the first test shall define `"a"` and `1`, the second `"b"` and 2, but for the third case,
/// a value for `v` exists (namely `"c"`), however no value to substitute for `w` exists.
/// Therefore, no fully valid set of tests can be constructed from the parameterized definition.
pub(crate) fn check_all_input_lengths(map: &IndexMap<syn::Ident, Vec<syn::Expr>>) -> usize {
    let mut arguments: Option<usize> = None;
    for (ident, values) in map.iter() {
        match arguments {
            Some(len) if len == values.len() => continue,
            None => arguments = Some(values.len()),
            _ => panic_on_inequal_length(map.iter(), ident),
        }
    }

    arguments.unwrap_or_default()
}

/// When this function gets invoked, it will construct an error message and then panic! with that message.
fn panic_on_inequal_length<K: Ord + Display, V, D: Display>(
    map: impl Iterator<Item = (K, V)>,
    ident: D,
) {
    let ids: String = map
        .map(|(id, _)| format!("{}", id))
        .collect::<Vec<String>>()
        .join(", ");

    panic!(
        "[parameterized-macro] error: Inconsistent argument list length for '{}'; all inputs ({}) should have equal length",
        ident,
        ids
    )
}
