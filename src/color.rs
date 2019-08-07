#[derive(Copy, Clone, Debug)]
pub struct Color(pub f32, pub f32, pub f32, pub f32);

impl Color {
    pub fn from_rgb_u32(r: u32, g: u32, b: u32) -> Self {
        Color(r as f32 / 256.0, g as f32 / 256.0, b as f32 / 256.0, 1.0)
    }
}

impl From<(u32, u32, u32)> for Color {
    fn from(tuple: (u32, u32, u32)) -> Self {
        Color::from_rgb_u32(tuple.0, tuple.1, tuple.2)
    }
}
