use crate::color::Color;
use crate::data::BlockData;
use crate::enums::Orientation;
use crate::level::Segment;

pub trait Blockish {
    fn get_color(&self) -> Color;
    fn get_orientation(&self) -> Orientation;

    // TODO: don't alloc/clone here?
    fn get_segments(&self) -> Vec<Segment>;
}

#[derive(Clone)]
pub struct Block {
    index: usize,
    pub movable: bool,
    position: (i32, i32),
    color: Color,
    orientation: Orientation,
    segments: Vec<Segment>,
}

impl Block {
    pub fn from_data(index: usize, data: &BlockData) -> Self {
        let movable = data.movable;
        let position = (data.position.0, data.position.1);
        let segments = data
            .segments
            .iter()
            .map(|segment| {
                let seg = Segment {
                    position: (segment[0], segment[1]),
                    shape: segment[2].into(),
                    board: segment[3].into(),
                };
                seg
            })
            .collect();
        let orientation = data.orientation.into();
        let color = Color::from_rgb_u32(data.color.0, data.color.1, data.color.2);

        Block {
            index,
            movable,
            position,
            color,
            segments,
            orientation,
        }
    }
}

impl Blockish for Block {
    fn get_color(&self) -> Color {
        self.color
    }

    fn get_orientation(&self) -> Orientation {
        self.orientation
    }

    fn get_segments(&self) -> Vec<Segment> {
        self.segments.clone()
    }
}
