#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/01-fails-non-enum.rs");
    t.pass("tests/02-simple.rs");
    t.pass("tests/03-with-fields.rs");
    t.compile_fail("tests/04-ignore-unknown-field.rs");
}
