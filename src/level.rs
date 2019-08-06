use std::collections::{HashMap, VecDeque};

use crate::data::LevelData;
use crate::enums::{Board, Orientation, PushDir, Shape};
use crate::renderer::Renderer;

pub struct Level {
    dimensions: (u32, u32),
    move_stack: VecDeque<()>,
    cell_map: CellMap,
    blocks: Vec<Block>,
    player1: Player,
    player2: Player,
}

#[derive(Clone)]
pub struct Block {
    movable: bool,
    position: (i32, i32),
    color: (f32, f32, f32),
    orientation: Orientation,
    segments: Vec<Segment>,
}

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct Segment(i32, i32, Shape, Board);

#[derive(Copy, Clone)]
pub struct Player {
    pub position: [i32; 2],
    pub color: [u32; 3],
}

impl Level {
    pub fn from_json(data: impl AsRef<str>) -> Level {
        let data: LevelData = json5::from_str(data.as_ref()).unwrap();
        println!("{:?}", data);

        let blocks = data
            .blocks
            .iter()
            .map(|block| {
                let movable = block.movable;
                let position = (block.position[0], block.position[1]);
                let segments = block
                    .segments
                    .iter()
                    .map(|segment| {
                        Segment(segment[0], segment[1], segment[2].into(), segment[3].into())
                    })
                    .collect();
                let orientation = block.orientation.into();
                let color = (
                    block.color[0] as f32 / 256.0,
                    block.color[1] as f32 / 256.0,
                    block.color[2] as f32 / 256.0,
                );
                Block {
                    movable,
                    position,
                    color,
                    segments,
                    orientation,
                }
            })
            .collect();

        let player1 = Player {
            position: data.player1.position,
            color: data.player1.color,
        };
        let player2 = Player {
            position: data.player2.position,
            color: data.player2.color,
        };

        Level {
            dimensions: (data.dimensions[0], data.dimensions[1]),
            move_stack: VecDeque::new(),
            cell_map: CellMap::new(),
            blocks,
            player1,
            player2,
        }
    }

    // player1: true -> player1, false -> player2
    // TODO: don't use a boolean here
    pub fn handle_movement(&mut self, player1: bool, direction: PushDir) -> bool {
        let mut player = if player1 {
            &self.player1
        } else {
            &self.player2
        };

        // TODO: check out of bounds
        let movement = direction.as_pair();
        let x = player.position[0] + movement.0;
        let y = player.position[1] + movement.1;

        let result = self.can_move(player1, direction).clone();
        let mut player = if player1 {
            &mut self.player1
        } else {
            &mut self.player2
        };

        if let Some(_) = result {
            player.position[0] = x;
            player.position[1] = y;
            true
        } else {
            false
        }
    }

    // TODO: don't use a boolean here
    pub fn can_move(&self, player1: bool, direction: PushDir) -> Option<()> {
        // an absolute segment (as opposed to relative to a block)
        #[derive(Copy, Clone, PartialOrd, PartialEq)]
        struct ASegment(i32, i32, Shape, Board);

        fn can_push(src: Segment, dst: Segment) -> bool {
            if src.3 != dst.3 {
                return false;
            }

            true
        }

        let player = if player1 {
            (
                self.player1.position[0],
                self.player1.position[1],
                Board::Left,
            )
        } else {
            (
                self.player2.position[0],
                self.player2.position[1],
                Board::Right,
            )
        };

        // check to make sure that the player isn't trying to go out of bounds
        let target = player + direction;
        if target.0 < 0
            || target.0 >= self.dimensions.0 as i32
            || target.1 < 0
            || target.1 >= self.dimensions.1 as i32
        {
            return None;
        }

        // check if we're sharing a triangle cell
        if let CellContents::Double(a, b) = self.cell_map.get(player) {}

        // 08/06 pickup
        // need to determine whether or not segment should hold a reference back to block or not?
        // either way, segment in the cellmap should hold block information
        // maybe cellmap should just carry a block index? seems hacky
        // using refs to manage the whole thing is messy and probably doesn't work
        // ???

        Some(())
    }

    pub fn render(&self, renderer: &mut Renderer) {
        // board positioning calculations
        let playfield_ratio = (2 * self.dimensions.0 + 6) as f32 / (self.dimensions.1 + 4) as f32;
        let screen_ratio = renderer.window.0 / renderer.window.1;

        let cols = self.dimensions.0 as i32;
        let rows = self.dimensions.1 as i32;

        let (scale, xoff, yoff) = if playfield_ratio > screen_ratio {
            let scale = renderer.window.0 as i32 / (2 * cols + 6);
            let yoff = renderer.window.1 as i32 / 2 - (rows + 4) * scale / 2;
            (scale, 0, yoff)
        } else {
            let scale = renderer.window.1 as i32 / (rows + 4);
            let xoff = renderer.window.0 as i32 / 2 - (2 * cols + 6) * scale / 2;
            (scale, xoff, 0)
        };

        self.render_boards(renderer, scale, (xoff, yoff));
    }

    fn render_boards(&self, renderer: &mut Renderer, scale: i32, offset: (i32, i32)) {
        let left_off = (offset.0 + 2 * scale, offset.1 + 2 * scale);
        let right_off = (
            offset.0 + (4 + self.dimensions.0 as i32) * scale,
            offset.1 + 2 * scale,
        );

        // render the grid
        // TODO: do this in one single pass instead of once for each cell
        for x in 0..self.dimensions.0 as i32 {
            for y in 0..self.dimensions.1 as i32 {
                renderer.render_cell((left_off.0 + x * scale, left_off.1 + y * scale), scale);
                renderer.render_cell((right_off.0 + x * scale, right_off.1 + y * scale), scale);
            }
        }

        // render blocks
        for block in self.blocks.iter() {
            for segment in block.segments.iter() {
                let offset = match &segment.3 {
                    Board::Left => left_off,
                    Board::Right => right_off,
                };
                let coord = (segment.0 + block.position.0, segment.1 + block.position.1);
                let location = (offset.0 + coord.0 * scale, offset.1 + coord.1 * scale);
                renderer.render_segment(location, scale, block.color, block.orientation, segment.2);
            }
        }

        // render player
        self.render_player(renderer, &self.player1, scale, left_off);
        self.render_player(renderer, &self.player2, scale, right_off);
    }

    fn render_player(
        &self,
        renderer: &mut Renderer,
        player: &Player,
        scale: i32,
        offset: (i32, i32),
    ) {
        let location = (
            offset.0 + player.position[0] * scale + 4,
            offset.1 + player.position[1] * scale + 4,
        );
        renderer.render_segment(
            location,
            (scale - 8),
            (
                player.color[0] as f32,
                player.color[1] as f32,
                player.color[2] as f32,
            ),
            Orientation::Both,
            Shape::Full,
        );
    }
}

struct CellMap(HashMap<(i32, i32, Board), CellContents>);

#[derive(Copy, Clone)]
enum CellContents {
    Empty,
    Player,
    Single(Segment),

    // invariant: .0 < .1
    Double(Segment, Segment),
}

impl CellMap {
    pub fn new() -> Self {
        CellMap(HashMap::new())
    }

    pub fn get(&self, loc: (i32, i32, Board)) -> CellContents {
        self.0
            .get(&loc)
            .cloned()
            .unwrap_or_else(|| CellContents::Empty)
    }

    pub fn clear(&mut self, loc: (i32, i32, Board)) {
        self.0.remove(&loc);
    }

    pub fn add(&mut self, loc: (i32, i32, Board), segment: &Segment) -> bool {
        let contents = self.get(loc).clone();
        match contents {
            CellContents::Empty => {
                // just add it like normal
                self.0.insert(loc, CellContents::Single(*segment));
                true
            }
            CellContents::Single(existing) => {
                if existing.2.is_opposite(&segment.2) {
                    self.0.insert(
                        loc,
                        if *segment < existing {
                            CellContents::Double(*segment, existing)
                        } else {
                            CellContents::Double(existing, *segment)
                        },
                    );
                    true
                } else {
                    false
                }
            }
            CellContents::Player | CellContents::Double(_, _) => false,
        }
    }
}
