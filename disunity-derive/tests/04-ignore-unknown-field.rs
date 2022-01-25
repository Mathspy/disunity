use disunity_derive::Variant;

#[derive(Variant)]
enum Class {
    Unknown(u32),
    #[disunity(discriminant = 1)]
    GameObject {
        field: i32,
    },
    #[disunity(discriminant = 2)]
    Transform,
}

fn main() {
    let _ = ClassVariant::GameObject;
    let _ = ClassVariant::Transform;
    let _ = ClassVariant::Unknown;
}
