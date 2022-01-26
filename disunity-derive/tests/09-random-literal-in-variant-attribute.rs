use disunity_derive::Variant;

#[derive(Variant)]
enum Class {
    #[disunity("hi from random valley")]
    Magic(i32),
}

#[derive(Variant)]
enum Other {
    #[disunity(discriminant = 4, "hi from random valley")]
    Magicks(i32),
}

fn main() {}
