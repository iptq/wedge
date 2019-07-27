use std::time::Duration;

use glium::{Display, Frame};

use crate::level::Level;
use crate::renderer::Renderer;
use crate::resources::Resources;

const SEGMENT_VERT: &str = include_str!("../shaders/segment.vert");
const SEGMENT_FRAG: &str = include_str!("../shaders/segment.frag");
const CELL_VERT: &str = include_str!("../shaders/cell.vert");
const CELL_FRAG: &str = include_str!("../shaders/cell.frag");

const SEGMENT_IMAGE: &[u8] = include_bytes!("../textures/segment.png");

const LEVEL_TUTORIAL: &str = include_str!("../levels/tutorial.json");

pub struct Game<'a> {
    pub resources: Resources,
    pub display: &'a Display,
    levels: Vec<Level>,
    current_level: usize,
}

impl<'a> Game<'a> {
    pub fn new(display: &'a Display) -> Game {
        let mut resources = Resources::default();
        resources
            .load_image_from_memory(display, "segment", &SEGMENT_IMAGE, false)
            .unwrap();
        resources
            .load_shader(display, "segment", &SEGMENT_VERT, &SEGMENT_FRAG)
            .unwrap();
        resources
            .load_shader(display, "cell", &CELL_VERT, &CELL_FRAG)
            .unwrap();

        let levels = vec![Level::from_json(&LEVEL_TUTORIAL)];
        Game {
            resources,
            display,
            levels,
            current_level: 0,
        }
    }

    pub fn create_renderer<'b>(&self, target: &'b mut Frame) -> Renderer<'b, '_> {
        Renderer::new(self, target)
    }

    pub fn get_current_level(&self) -> &Level {
        self.levels.iter().nth(self.current_level).unwrap()
    }

    pub fn update(&mut self, delta: Duration) {}

    pub fn render(&self, renderer: &mut Renderer) {
        let level = self.get_current_level();
        level.render(renderer);
    }
}
