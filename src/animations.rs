use std::collections::HashMap;
use std::time::Duration;

pub type BlockOffsets = HashMap<usize, (i32, i32)>;

// TODO: don't yeet around a HashMap all the time
pub type AnimationFn = Box<Fn(BlockOffsets, f32) -> BlockOffsets>;

const ANIMATION_DURATION: f32 = 1.0 / 6.0;

#[derive(Default, Serialize)]
pub struct AnimationState {
    pub is_animating: bool,
    pub last_move_success: bool,
    pub progress: f32,
    pub block_offsets: BlockOffsets,
    #[serde(skip)]
    progress_function: Option<AnimationFn>,
}

impl AnimationState {
    pub fn new() -> Self {
        AnimationState {
            is_animating: false,
            last_move_success: true,
            progress: 0.0,
            block_offsets: BlockOffsets::new(),
            progress_function: None,
        }
    }

    pub fn begin_transition(&mut self, f: AnimationFn) {
        self.is_animating = true;
        self.progress = 0.0;
        self.progress_function = Some(f);
    }

    pub fn make_progress(&mut self, delta: Duration) {
        let progress = self.progress + delta.as_millis() as f32 / 1000.0;
        let block_offsets = if let Some(f) = &self.progress_function {
            Some(f(self.block_offsets.clone(), progress))
        } else {
            None
        };
        if let Some(block_offsets) = block_offsets {
            self.block_offsets = block_offsets;
            self.progress = progress;
        }
    }

    pub fn is_done(&self) -> bool {
        self.progress > 1.0
    }

    pub fn get_offset(&self, index: usize) -> (i32, i32) {
        self.block_offsets
            .get(&index)
            .cloned()
            .unwrap_or_else(|| (0, 0))
    }
}
