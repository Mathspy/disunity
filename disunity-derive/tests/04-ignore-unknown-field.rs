use disunity_derive::Variants;

#[derive(Variants)]
enum Class {
    Unknown(u32),
    GameObject { field: i32 },
    Transform,
}

fn main() {
    let _ = ClassVariants::GameObject;
    let _ = ClassVariants::Transform;
    let _ = ClassVariants::Unknown;
}
