use disunity_derive::Variants;

#[derive(Variants)]
enum Class {
    #[disunity(discriminant = 1)]
    GameObject { field: i32 },
}

fn main() {
    let _ = ClassVariants::GameObject;
}
