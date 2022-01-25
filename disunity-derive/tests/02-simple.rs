use disunity_derive::Variants;

#[derive(Variants)]
enum Class {
    #[disunity(discriminant = 1)]
    GameObject,
}

fn main() {
    let _ = ClassVariants::GameObject;
}
