use disunity_derive::Variant;

#[derive(Variant)]
enum Class {
    #[disunity(discriminant = "nonsense")]
    GameObject { field: i32 },
}

fn main() {
    let _ = ClassVariant::GameObject;
}
