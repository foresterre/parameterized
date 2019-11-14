#[cfg(feature = "enable_test_with_wildcard")]
#[test]
fn expected_failures() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fail/*.rs");
}

#[cfg(feature = "enable_test_with_wildcard")]
#[test]
fn expected_ok_all() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ok/*.rs");
}

#[cfg(not(feature = "enable_test_with_wildcard"))]
#[test]
fn individual_cases() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ok/01_import.rs");
    t.pass("tests/ok/02_multiple_ids.rs");
    t.pass("tests/ok/03_multiline.rs");
    t.pass("tests/ok/04_many_arg.rs");

    t.pass("tests/ok/06_vis.rs");
    t.pass("tests/ok/07_vis2.rs");
    t.pass("tests/ok/08_neg.rs");
    t.pass("tests/ok/09_option.rs");
    t.pass("tests/ok/10_result.rs");
    t.pass("tests/ok/11_enum.rs");
    t.pass("tests/ok/12_enum_with_variant_value.rs");
    t.pass("tests/ok/13_import_rename.rs");
    t.pass("tests/ok/14_transitive_attr.rs");

    t.compile_fail("tests/fail/id_already_defined.rs");
    t.compile_fail("tests/fail/inequal_amount_of_arg.rs");
    t.compile_fail("tests/fail/not_a_fn.rs");
    t.compile_fail("tests/fail/on_visibility.rs");
    t.compile_fail("tests/fail/square_brackets.rs");
}
