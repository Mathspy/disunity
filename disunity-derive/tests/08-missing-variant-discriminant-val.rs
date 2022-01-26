use disunity_derive::Variant;

#[derive(Variant)]
enum Class {
    #[disunity(discriminant)]
    Magic(i32),
}

fn main() {}
