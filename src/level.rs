use std::collections::VecDeque;

use crate::data::LevelData;
use crate::enums::{Board, Shape};
use crate::renderer::Renderer;
use crate::{GAME_HEIGHT, GAME_WIDTH};

pub struct Level {
    dimensions: (u32, u32),
    move_stack: VecDeque<()>,
    blocks: Vec<Block>,
}

#[derive(Clone)]
pub struct Block {
    segments: Vec<Segment>,
}

#[derive(Copy, Clone)]
pub struct Segment(u32, u32, Shape, Board);

impl Level {
    pub fn from_json(data: impl AsRef<str>) -> Level {
        let data: LevelData = json5::from_str(data.as_ref()).unwrap();
        println!("{:?}", data);

        let blocks = data
            .blocks
            .iter()
            .map(|block| {
                let segments = block
                    .segments
                    .iter()
                    .map(|segment| {
                        Segment(segment[0], segment[1], segment[2].into(), segment[3].into())
                    })
                    .collect();
                Block { segments }
            })
            .collect();

        Level {
            dimensions: (data.dimensions[0], data.dimensions[1]),
            move_stack: VecDeque::new(),
            blocks,
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        let playfield_ratio = (2 * self.dimensions.0 + 6) as f64 / (self.dimensions.1 + 4) as f64;
        let screen_ratio = GAME_WIDTH as f64 / GAME_HEIGHT as f64;

        let (scale, xoff, yoff) = if playfield_ratio > screen_ratio {
            let scale = GAME_WIDTH / (2 * self.dimensions.0 + 6);
            let yoff = GAME_HEIGHT / 2 - (self.dimensions.1 + 4) * scale / 2;
            (scale, 0, yoff)
        } else {
            let scale = GAME_HEIGHT / (self.dimensions.1 + 4);
            let xoff = GAME_WIDTH / 2 - (2 * self.dimensions.0 + 6) * scale / 2;
            (scale, xoff, 0)
        };

        self.render_boards(renderer, scale, (xoff, yoff));
    }

    fn render_boards(&self, renderer: &mut Renderer, scale: u32, offset: (u32, u32)) {
        let left_off = (offset.0 + 2 * scale, offset.1 + 2 * scale);
        let right_off = (
            offset.0 + (4 + self.dimensions.0) * scale,
            offset.1 + 2 * scale,
        );

        renderer.render_cell(left_off, 50, Shape::Full);
    }
}
