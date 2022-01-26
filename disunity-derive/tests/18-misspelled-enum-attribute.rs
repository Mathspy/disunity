use disunity_derive::Variant;

#[derive(Variant)]
#[disunity(misspelled = u8)]
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

fn main() {}
