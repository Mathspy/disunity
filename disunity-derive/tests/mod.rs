#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/01-fails-non-enum.rs");
}
