use disunity_derive::Variant;

#[derive(Variant)]
#[disunity(discriminant = u32)]
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
    assert_eq!(ClassVariant::from_int(0u32), None);
    assert_eq!(ClassVariant::from_int(1u32), Some(ClassVariant::GameObject));
    assert_eq!(ClassVariant::from_int(2u32), Some(ClassVariant::Transform));
    assert_eq!(ClassVariant::from_int(3u32), Some(ClassVariant::Magic));
    assert_eq!(ClassVariant::from_int(4u32), None);
}
