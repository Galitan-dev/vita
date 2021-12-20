use rand::{random};

pub fn random_color() -> [f32; 3] {
    [
        random::<f32>(),
        random::<f32>(),
        random::<f32>()
    ]
}