use disunity_derive::Variant;

#[derive(Variant)]
struct Class {
    #[disunity(discriminant = 1)]
    field: i32,
}

#[derive(Variant)]
union MyUnion {
    #[disunity(discriminant = 1)]
    f1: u32,
    #[disunity(discriminant = 2)]
    f2: f32,
}

fn main() {}
