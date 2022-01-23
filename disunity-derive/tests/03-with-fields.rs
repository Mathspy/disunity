use disunity_derive::Variants;

#[derive(Variants)]
enum Class {
    GameObject { field: i32 },
}

fn main() {
    let _ = ClassVariants::GameObject;
}
