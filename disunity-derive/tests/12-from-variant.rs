use disunity_derive::Variant;

#[derive(Debug, PartialEq, Variant)]
enum Class {
    // Unknown is special and doesn't need a discriminant
    Unknown(isize),
    #[disunity(discriminant = 1)]
    GameObject {
        field: i32,
    },
    #[disunity(discriminant = 2)]
    Transform,
    #[disunity(discriminant = 3)]
    Magic(i32),
}

fn main() {
    assert_eq!(Class::from_variant(ClassVariant::GameObject), None);
    assert_eq!(
        Class::from_variant(ClassVariant::Transform),
        Some(Class::Transform)
    );
    assert_eq!(Class::from_variant(ClassVariant::Magic), None);
}
