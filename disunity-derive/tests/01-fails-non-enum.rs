use disunity_derive::Variants;

#[derive(Variants)]
struct Class {
    field: i32,
}

#[derive(Variants)]
union MyUnion {
    f1: u32,
    f2: f32,
}

fn main() {}
