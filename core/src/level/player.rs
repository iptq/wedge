use crate::color::Color;
use crate::enums::Orientation;
use crate::level::{Blockish, Segment};

#[derive(Copy, Clone)]
pub struct Player {
    pub position: (i32, i32),
    pub color: Color,
}

impl Blockish for Player {
    fn get_color(&self) -> Color {
        self.color
    }

    fn get_orientation(&self) -> Orientation {
        Orientation::None
    }

    fn get_segments(&self) -> Vec<Segment> {
        vec![]
    }
}
