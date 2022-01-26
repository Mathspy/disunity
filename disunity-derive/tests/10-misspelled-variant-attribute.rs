use disunity_derive::Variant;

#[derive(Variant)]
enum Class {
    #[disunity(misspelled = 3)]
    Magic(i32),
}

fn main() {}
