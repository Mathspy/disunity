use disunity_derive::Variant;

#[derive(Variant)]
enum Class {
    #[disunity(discriminant = 1)]
    GameObject,
}

fn main() {
    let _ = ClassVariant::GameObject;
}
