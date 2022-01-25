use disunity_derive::Variant;

#[derive(Variant)]
struct Class {
    #[disunity(discriminate = 1)]
    field: i32,
}

#[derive(Variant)]
union MyUnion {
    #[disunity(discriminate = 1)]
    f1: u32,
    #[disunity(discriminate = 2)]
    f2: f32,
}

fn main() {}
