#[macro_use]
mod macros;

mod block;
mod player;

use std::collections::{HashMap, HashSet, VecDeque};

use crate::animations::AnimationState;
use crate::color::Color;
use crate::data::LevelData;
use crate::enums::{Board, Orientation, PushDir, Shape};
use crate::renderer::Renderer;

use self::block::{Block, Blockish};
use self::player::Player;

pub struct Level {
    dimensions: (u32, u32),
    move_stack: VecDeque<()>,
    blocks: Vec<Block>,
    player1: Player,
    player2: Player,
    goal1: (i32, i32),
    goal2: (i32, i32),
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Segment {
    position: (i32, i32),
    shape: Shape,
    board: Board,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Entity {
    Block(usize),
    Player(Board),
}

pub type ChangeSet = HashMap<Entity, PushDir>;

pub type FailSet = HashSet<usize>;

impl Level {
    pub fn from_json(data: impl AsRef<str>) -> Level {
        let data: LevelData = json5::from_str(data.as_ref()).unwrap();
        println!("level data: {:?}", data);

        let blocks = data
            .blocks
            .iter()
            .enumerate()
            .map(|(i, block)| Block::from_data(i, block))
            .collect();

        let player1 = Player {
            position: data.player1.position,
            color: data.player1.color.into(),
        };
        let player2 = Player {
            position: data.player2.position,
            color: data.player2.color.into(),
        };

        Level {
            dimensions: (data.dimensions[0], data.dimensions[1]),
            move_stack: VecDeque::new(),
            blocks,
            player1,
            player2,
            goal1: data.goal1,
            goal2: data.goal2,
        }
    }

    // check if we won
    pub fn check_win_condition(&self) -> bool {
        self.player1.position == self.goal1 && self.player2.position == self.goal2
    }

    pub fn apply_change_set(&mut self, change_set: ChangeSet) {
        for (entity, direction) in change_set {
            let direction = direction.as_pair();
            match entity {
                Entity::Player(board) => {
                    let player = match board {
                        Board::Left => &mut self.player1,
                        Board::Right => &mut self.player2,
                    };
                    player.position.0 += direction.0;
                    player.position.1 += direction.1;
                }
                Entity::Block(index) => {
                    let block = self.blocks.get_mut(index).expect("big failure");
                    for segment in &mut block.segments {
                        segment.position.0 += direction.0;
                        segment.position.1 += direction.1;
                    }
                }
            }
        }
    }

    pub fn try_move(&mut self, board: Board, direction: PushDir) -> Result<ChangeSet, FailSet> {
        let mut change_set = ChangeSet::default();
        change_set.insert(Entity::Player(board), direction);
        self.player_can_move(board, direction, change_set)
    }

    fn player_can_move(
        &self,
        board: Board,
        direction: PushDir,
        change_set: ChangeSet,
    ) -> Result<ChangeSet, FailSet> {
        let player = match board {
            Board::Left => &self.player1,
            Board::Right => &self.player2,
        };
        let player_segment = Segment {
            position: player.position,
            shape: Shape::Full,
            board,
        };
        self.segment_can_move(None, player_segment, direction, change_set)
    }

    fn block_can_move(
        &self,
        index: usize,
        direction: PushDir,
        mut change_set: ChangeSet,
    ) -> Result<ChangeSet, FailSet> {
        println!("block_can_move({:?}, {:?})", index, direction);
        let block = match self.blocks.get(index) {
            Some(block) => block,
            None => return Err(HashSet::new()),
        };

        // is the block even movable?
        if !block.movable {
            return Err(set!(index));
        }

        // does the direction match the orientation?
        match (block.orientation, direction) {
            (Orientation::Horizontal, PushDir::Left)
            | (Orientation::Horizontal, PushDir::Right)
            | (Orientation::Vertical, PushDir::Up)
            | (Orientation::Vertical, PushDir::Down)
            | (Orientation::Both, _) => (),
            _ => return Err(set!(index)),
        }

        // TODO: change this to use &mut instead of returning a new one each time
        change_set.insert(Entity::Block(index), direction);
        for segment in block.get_segments() {
            match self.segment_can_move(Some(index), segment, direction, change_set.clone()) {
                Ok(new_change_set) => change_set = new_change_set,
                Err(fail_set) => return Err(fail_set),
            }
        }

        Ok(change_set)
    }

    fn segment_can_move(
        &self,
        block_index: Option<usize>,
        segment: Segment,
        direction: PushDir,
        change_set: ChangeSet,
    ) -> Result<ChangeSet, FailSet> {
        println!(
            "segment_can_move({:?}, {:?}, {:?})",
            block_index, segment, direction
        );
        let segment_loc = (segment.position.0, segment.position.1, segment.board);
        let target = segment_loc + direction;

        // is the target actually in the map?
        if target.0 < 0
            || target.0 >= self.dimensions.0 as i32
            || target.1 < 0
            || target.1 >= self.dimensions.1 as i32
        {
            return Err(entity_fail!(block_index));
        }

        // retrieve other blocks that might be occupying this current space and the target space
        let mut current_occupant = None;
        let mut target_occupant = None;
        for (i, block) in self.blocks.iter().enumerate() {
            // skip other segments of the same block
            if let Some(n) = block_index {
                if n == i {
                    continue;
                }
            }

            // offset from the change set
            let offset = match change_set.get(&Entity::Block(i)) {
                Some(direction) => direction.as_pair(),
                None => (0, 0),
            };

            for segment in block.get_segments() {
                // don't get segments on different boards
                if segment.board != segment_loc.2 {
                    continue;
                }

                let mut segment_pos = segment.position;
                segment_pos.0 += offset.0;
                segment_pos.1 += offset.1;

                if segment_pos == (segment_loc.0, segment_loc.1) {
                    current_occupant = Some((i, segment.shape));
                }
                if segment_pos == (target.0, target.1) {
                    target_occupant = Some((Entity::Block(i), segment.shape));
                }
            }
        }

        println!(
            "  occupants: {:?} | {:?}",
            current_occupant, target_occupant
        );

        // handle special pushes
        if let Some((other_block, other_shape)) = current_occupant {
            // are both shapes triangles?
            let both_triangles = match (segment.shape, other_shape) {
                (Shape::Full, Shape::Full) => false,
                _ => true,
                // TODO: enumerate them to get rid of invalid states
            };

            if both_triangles {
                // what directions could we be pushing the other block into?
                let possible_directions = match segment.shape {
                    Shape::TopRight => [PushDir::Up, PushDir::Right],
                    Shape::TopLeft => [PushDir::Left, PushDir::Up],
                    Shape::BottomLeft => [PushDir::Down, PushDir::Left],
                    Shape::BottomRight => [PushDir::Right, PushDir::Down],
                    Shape::Full => unreachable!("already eliminated this possibility"),
                };

                // does the direction we're pushing appear in this list?
                if possible_directions.contains(&direction) {
                    // the other shape goes in the other direction
                    let other_direction = {
                        let mut set = possible_directions.iter().collect::<HashSet<_>>();
                        set.remove(&direction);
                        *set.into_iter().next().unwrap()
                    };

                    return self.block_can_move(other_block, other_direction, change_set);
                }
            }
        }

        // handle normal pushes
        if let Some((entity, shape)) = target_occupant {
            match entity {
                Entity::Player(_) => {
                    // TODO: assert that the board is the same
                    Err(fail_set!(change_set))
                }
                Entity::Block(index) => {
                    if
                    // if it's part of the same block it's ok to push
                    block_index.is_some() && block_index.unwrap() == index ||
                    // if the shapes are opposite, we can actually both fit into the same spot
                    segment.shape.is_opposite(shape)
                    {
                        Ok(change_set)
                    }
                    // if the block is already in the change set, it can't move
                    else if change_set.contains_key(&Entity::Block(index)) {
                        Err(fail_set!(change_set))
                    }
                    // if the next block can move then so can this one
                    else {
                        self.block_can_move(index, direction, change_set)
                    }
                }
            }
        } else {
            // coast is clear, push away!
            Ok(change_set)
        }
    }

    pub fn render(&self, renderer: &mut Renderer, animations: &AnimationState) {
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

        self.render_boards(renderer, scale, (xoff, yoff), animations);
    }

    fn render_boards(
        &self,
        renderer: &mut Renderer,
        scale: i32,
        offset: (i32, i32),
        animations: &AnimationState,
    ) {
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
        for (i, block) in self.blocks.iter().enumerate() {
            for segment in block.get_segments().iter() {
                let offset = match &segment.board {
                    Board::Left => left_off,
                    Board::Right => right_off,
                };
                let mut location = (
                    offset.0 + segment.position.0 * scale,
                    offset.1 + segment.position.1 * scale,
                );
                let animation_offset = animations.get_block_offset(i);
                location.0 += (animation_offset.0 * scale as f32) as i32;
                location.1 += (animation_offset.1 * scale as f32) as i32;
                renderer.render_segment(
                    location,
                    scale,
                    block.get_color(),
                    block.get_orientation(),
                    segment.shape,
                );
            }
        }

        // render goals
        self.render_goal(renderer, self.goal1, scale, left_off);
        self.render_goal(renderer, self.goal2, scale, right_off);

        // render player
        self.render_player(
            renderer,
            Board::Left,
            &self.player1,
            scale,
            animations,
            left_off,
        );
        self.render_player(
            renderer,
            Board::Right,
            &self.player2,
            scale,
            animations,
            right_off,
        );
    }

    fn render_player(
        &self,
        renderer: &mut Renderer,
        board: Board,
        player: &Player,
        scale: i32,
        animations: &AnimationState,
        offset: (i32, i32),
    ) {
        let mut location = (
            offset.0 + player.position.0 * scale + 4,
            offset.1 + player.position.1 * scale + 4,
        );
        let animation_offset = animations.get_player_offset(board);
        location.0 += (animation_offset.0 * scale as f32) as i32;
        location.1 += (animation_offset.1 * scale as f32) as i32;
        renderer.render_segment(
            location,
            scale - 8,
            player.color,
            Orientation::Both,
            Shape::Full,
        );
    }

    fn render_goal(
        &self,
        renderer: &mut Renderer,
        location: (i32, i32),
        scale: i32,
        offset: (i32, i32),
    ) {
        let position = (
            offset.0 + location.0 * scale + 4,
            offset.1 + location.1 * scale + 4,
        );
        renderer.render_segment(
            position,
            scale - 8,
            Color::from_rgb_u32(102, 204, 102),
            Orientation::Both,
            Shape::Full,
        );
    }
}
