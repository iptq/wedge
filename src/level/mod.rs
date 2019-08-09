mod block;
mod cell_map;
mod player;

use std::collections::{HashMap, VecDeque};

use crate::color::Color;
use crate::data::LevelData;
use crate::enums::{Board, Orientation, PushDir, Shape};
use crate::renderer::Renderer;

use self::block::{Block, Blockish};
use self::cell_map::{CellContents, CellMap};
use self::player::Player;

pub struct Level {
    dimensions: (u32, u32),
    move_stack: VecDeque<()>,
    cell_map: CellMap,
    blocks: Vec<Block>,
    player1: Player,
    player2: Player,
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Segment {
    position: (i32, i32),
    shape: Shape,
    board: Board,
}

impl Level {
    pub fn from_json(data: impl AsRef<str>) -> Level {
        let data: LevelData = json5::from_str(data.as_ref()).unwrap();
        println!("{:?}", data);

        let mut cell_map = CellMap::new();

        let blocks = data
            .blocks
            .iter()
            .enumerate()
            .map(|(i, block)| {
                let block = Block::from_data(i, block);
                for segment in block.get_segments() {
                    cell_map.add(
                        (segment.position.0, segment.position.1, segment.board),
                        i,
                        &segment,
                    );
                }
                block
            })
            .collect();

        let player1 = Player {
            position: data.player1.position,
            color: data.player1.color.into(),
        };
        let player2 = Player {
            position: data.player2.position,
            color: data.player2.color.into(),
        };
        cell_map.add_player((
            data.player1.position.0,
            data.player1.position.1,
            Board::Left,
        ));
        cell_map.add_player((
            data.player2.position.0,
            data.player2.position.1,
            Board::Right,
        ));

        Level {
            dimensions: (data.dimensions[0], data.dimensions[1]),
            move_stack: VecDeque::new(),
            cell_map,
            blocks,
            player1,
            player2,
        }
    }

    // player1: true -> player1, false -> player2
    // TODO: don't use a boolean here
    pub fn handle_movement(&mut self, player1: bool, direction: PushDir) -> bool {
        let player = if player1 {
            &self.player1
        } else {
            &self.player2
        };

        // TODO: check out of bounds
        let movement = direction.as_pair();
        let x = player.position.0 + movement.0;
        let y = player.position.1 + movement.1;

        let result = self.can_move(player1, direction).clone();
        let mut player = if player1 {
            &mut self.player1
        } else {
            &mut self.player2
        };

        if let Some(_) = result {
            player.position.0 = x;
            player.position.1 = y;
            true
        } else {
            false
        }
    }

    pub fn try_move(&self) {}

    fn block_can_move(&self, block: impl Blockish) {
        for segment in block.get_segments() {}
    }

    fn segment_can_move(&self, block: Block, segment: Segment, direction: PushDir) -> Option<()> {
        let triple = (segment.position.0, segment.position.1, segment.board);
        let target = triple + direction;

        // is the target in the map?
        if target.0 < 0
            || target.0 >= self.dimensions.0 as i32
            || target.1 < 0
            || target.1 >= self.dimensions.1 as i32
        {
            return None;
        }

        // check if we're sharing a triangle cell
        if let CellContents::Double((ind1, block1), (ind2, block2)) = self.cell_map.get(triple) {
            // figure out which one is the other block

            // check that we're pushing in the direction of the other block

        }

        Some(())
    }

    // TODO: don't use a boolean here
    pub fn can_move(&self, player1: bool, direction: PushDir) -> Option<()> {
        // an absolute segment (as opposed to relative to a block)
        #[derive(Copy, Clone, PartialOrd, PartialEq)]
        struct ASegment(i32, i32, Shape, Board);

        struct PushMap(CellMap);

        fn can_push_segment(src: ASegment, dst: ASegment) -> bool {
            if src.3 != dst.3 {
                return false;
            }

            true
        }

        let player = if player1 {
            (
                self.player1.position.0,
                self.player1.position.1,
                Board::Left,
            )
        } else {
            (
                self.player2.position.0,
                self.player2.position.1,
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
        if let CellContents::Double(a, b) = self.cell_map.get(player) {
            // get the shape of the other block
        }

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
            for segment in block.get_segments().iter() {
                let offset = match &segment.board {
                    Board::Left => left_off,
                    Board::Right => right_off,
                };
                let location = (
                    offset.0 + segment.position.0 * scale,
                    offset.1 + segment.position.1 * scale,
                );
                renderer.render_segment(
                    location,
                    scale,
                    block.get_color(),
                    block.get_orientation(),
                    segment.shape,
                );
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
            offset.0 + player.position.0 * scale + 4,
            offset.1 + player.position.1 * scale + 4,
        );
        renderer.render_segment(
            location,
            (scale - 8),
            player.color,
            Orientation::Both,
            Shape::Full,
        );
    }
}
