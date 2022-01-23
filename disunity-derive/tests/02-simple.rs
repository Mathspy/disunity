use disunity_derive::Variants;

#[derive(Variants)]
enum Class {
    GameObject,
}

fn main() {
    let _ = ClassVariants::GameObject;
}
