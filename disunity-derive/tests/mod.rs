#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/01-non-enum.rs");
    t.pass("tests/02-simple.rs");
    t.pass("tests/03-with-fields.rs");
    t.compile_fail("tests/04-ignore-unknown-field.rs");
    t.compile_fail("tests/05-invalid-variant-discriminant.rs");
    t.compile_fail("tests/06-too-many-variant-attributes.rs");
    t.compile_fail("tests/07-missing-variant-discriminant.rs");
    t.compile_fail("tests/08-missing-variant-discriminant-val.rs");
    t.compile_fail("tests/09-random-literal-in-variant-attribute.rs");
    t.compile_fail("tests/10-misspelled-variant-attribute.rs");
    t.pass("tests/11-from-integer.rs");
    t.pass("tests/12-from-variant.rs");
    t.pass("tests/13-discriminant-type.rs");
    t.pass("tests/14-discriminant-type-default.rs");
    t.compile_fail("tests/15-invalid-discriminant-type.rs");
    t.compile_fail("tests/16-invalid-enum-attribute.rs");
    t.compile_fail("tests/17-too-many-enum-attributes.rs");
    t.compile_fail("tests/18-misspelled-enum-attribute.rs");
}
