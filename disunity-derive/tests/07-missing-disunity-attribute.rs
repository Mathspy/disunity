use disunity_derive::Variant;

#[derive(Variant)]
enum Class {
    // Unknown is special and doesn't need a discriminant
    Unknown(i32),
    #[disunity(discriminant = 1)]
    GameObject {
        field: i32,
    },
    Transform,
    Magic(i32),
}

fn main() {}
