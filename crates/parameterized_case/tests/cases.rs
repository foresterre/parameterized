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

    t.pass("tests/ok/01_case.rs");
    t.pass("tests/ok/02_multiple_parameters.rs");
}
