use std::collections::HashMap;

type BlockOffsets = HashMap<usize, (u32, u32)>;
type AnimationFn = Box<Fn(BlockOffsets, f32) -> BlockOffsets>;

#[derive(Default)]
pub struct AnimationState {
    pub is_animating: bool,
    pub last_move_success: bool,
    pub progress: f32,
    pub block_offsets: BlockOffsets,

    progress_function: Option<AnimationFn>,
}

impl AnimationState {
    pub fn begin_transition(&mut self, f: AnimationFn) {
        self.is_animating = true;
        self.progress = 0.0;
        self.progress_function = Some(f);
    }
}
