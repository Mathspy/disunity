use disunity_derive::Variant;

#[derive(Variant)]
enum Class {
    #[disunity(discriminant = 4)]
    #[disunity(discriminant = 4)]
    GameObject { field: i32 },
}

fn main() {
    let _ = ClassVariant::GameObject;
}
