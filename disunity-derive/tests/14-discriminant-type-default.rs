use disunity_derive::Variant;

#[derive(Variant)]
// Defaults to isize discriminant type if none is specified
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
    assert_eq!(ClassVariant::from_int(0isize), None);
    assert_eq!(
        ClassVariant::from_int(1isize),
        Some(ClassVariant::GameObject)
    );
    assert_eq!(
        ClassVariant::from_int(2isize),
        Some(ClassVariant::Transform)
    );
    assert_eq!(ClassVariant::from_int(3isize), Some(ClassVariant::Magic));
    assert_eq!(ClassVariant::from_int(4isize), None);
}
