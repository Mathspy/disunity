use disunity_derive::Variant;

#[derive(Variant)]
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
    assert_eq!(ClassVariant::from_int(0), None);
    assert_eq!(ClassVariant::from_int(1), Some(ClassVariant::GameObject));
    assert_eq!(ClassVariant::from_int(2), Some(ClassVariant::Transform));
    assert_eq!(ClassVariant::from_int(3), Some(ClassVariant::Magic));
    assert_eq!(ClassVariant::from_int(4), None);
}
