use crate::data::LevelData;
use crate::renderer::Renderer;
use crate::screens::Screen;

pub struct EditorScreen {
    level: LevelData,
}

impl Screen for EditorScreen {
    fn render(&self, renderer: &mut Renderer) {}
}

impl EditorScreen {
    pub fn new() -> EditorScreen {
        EditorScreen {
            level: LevelData::empty(),
        }
    }
}
