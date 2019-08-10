use std::collections::HashMap;
use std::time::Duration;

use crate::enums::Board;
use crate::level::{ChangeSet, Entity, FailSet};

pub type MoveResult = Result<ChangeSet, FailSet>;
pub type BlockOffsets = HashMap<Entity, (f32, f32)>;

// TODO: don't yeet around a HashMap all the time
pub type AnimationFn = Box<Fn(MoveResult, BlockOffsets, f32) -> BlockOffsets>;

// in seconds
const ANIMATION_DURATION: f32 = 1.0 / 6.0;

#[derive(Default)]
pub struct AnimationState {
    pub is_animating: bool,
    pub last_move_result: Option<MoveResult>,
    pub progress: f32,
    pub block_offsets: BlockOffsets,
    progress_function: Option<AnimationFn>,
}

impl AnimationState {
    pub fn new() -> Self {
        AnimationState {
            is_animating: false,
            last_move_result: None,
            progress: 0.0,
            block_offsets: BlockOffsets::new(),
            progress_function: None,
        }
    }

    pub fn begin_move_transition(&mut self, result: MoveResult) {
        println!("result: {:?}", result);
        self.last_move_result = Some(result);
        self.is_animating = true;
        self.progress = 0.0;
        let func = |last_move_result: MoveResult, mut offsets: BlockOffsets, progress: f32| {
            use std::f32::consts::PI;
            match last_move_result {
                // transition
                Ok(change_set) => {
                    for (entity, direction) in change_set {
                        // TODO: implement ease-out?
                        let pair = direction.as_pair();
                        // cap progress at 1.0, we don't want blocks going past where they're supposed to
                        let progress = progress.min(1.0);
                        let offset = (pair.0 as f32 * progress, pair.1 as f32 * progress);
                        offsets.insert(entity, offset);
                    }
                }
                // vibrate all blocking pieces
                Err(fail_set) => {
                    for index in fail_set {
                        let delta = 0.05 * (4.0 * PI * progress).sin() / (progress + 0.5);
                        offsets.insert(Entity::Block(index), (delta, delta));
                    }
                }
            }
            offsets
        };
        self.progress_function = Some(Box::new(func));
    }

    pub fn make_progress(&mut self, delta: Duration) {
        let progress = self.progress + (delta.as_millis() as f32 / ANIMATION_DURATION) / 1000.0;

        let block_offsets = if let Some(f) = &self.progress_function {
            Some(f(
                self.last_move_result.clone().unwrap(),
                self.block_offsets.clone(),
                progress,
            ))
        } else {
            None
        };

        // this should always work
        if let Some(block_offsets) = block_offsets {
            self.block_offsets = block_offsets;
            self.progress = progress;
        }

        if self.progress > 1.0 {
            self.is_animating = false;
            self.block_offsets = BlockOffsets::new();
        }
    }

    pub fn get_block_offset(&self, index: usize) -> (f32, f32) {
        self.block_offsets
            .get(&Entity::Block(index))
            .cloned()
            .unwrap_or_else(|| (0.0, 0.0))
    }

    pub fn get_player_offset(&self, board: Board) -> (f32, f32) {
        self.block_offsets
            .get(&Entity::Player(board))
            .cloned()
            .unwrap_or_else(|| (0.0, 0.0))
    }
}
