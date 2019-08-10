use std::collections::HashMap;
use std::time::Duration;

use glium::glutin::{ElementState, Event, VirtualKeyCode, WindowEvent};
use glium::{Display, Frame};

use crate::animations::AnimationState;
use crate::enums::{Board, PushDir};
use crate::level::Level;
use crate::renderer::Renderer;
use crate::resources::Resources;

const SEGMENT_VERT: &str = include_str!("../shaders/segment.vert");
const SEGMENT_FRAG: &str = include_str!("../shaders/segment.frag");
const CELL_VERT: &str = include_str!("../shaders/cell.vert");
const CELL_FRAG: &str = include_str!("../shaders/cell.frag");

const SEGMENT_IMAGE: &[u8] = include_bytes!("../textures/segment.png");

const LEVEL_TUTORIAL: &str = include_str!("../levels/tutorial.json");

const ANIMATION_SPEED: f32 = 6.0;

pub struct Game<'a> {
    pub resources: Resources,
    pub display: &'a Display,
    levels: Vec<Level>,
    current_level: usize,
    keymap: HashMap<VirtualKeyCode, bool>,
    animations: AnimationState,
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
            keymap: HashMap::new(),
            animations: AnimationState::new(),
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => self.resources.window_dimensions = size.into(),
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(code) = &input.virtual_keycode {
                        if let ElementState::Pressed = &input.state {
                            self.keymap.insert(*code, true);
                        } else {
                            self.keymap.insert(*code, false);
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }

    pub fn create_renderer<'b>(&self, target: &'b mut Frame) -> Renderer<'b, '_> {
        Renderer::new(self, target)
    }

    pub fn get_current_level(&self) -> &Level {
        self.levels.iter().nth(self.current_level).unwrap()
    }

    pub fn get_current_level_mut(&mut self) -> &mut Level {
        self.levels.iter_mut().nth(self.current_level).unwrap()
    }

    pub fn is_pressed(&self, code: &VirtualKeyCode) -> bool {
        if let Some(true) = self.keymap.get(code) {
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, delta: Duration) {
        macro_rules! shit {
            ($key:expr, $board:expr, $direction:expr) => {
                if self.is_pressed(&$key) {
                    println!("pushed: {:?}", $key);
                    let level = self.get_current_level_mut();
                    let result = level.try_move($board, $direction);
                    println!("game result: {:?}", result);
                    self.keymap.insert($key, false);
                }
            };
        }

        if self.animations.is_animating {
            if self.animations.last_move_success {
                if self.animations.is_done() {
                    self.animations.make_progress(delta);
                }
            } else {
            }
        } else {
            shit!(VirtualKeyCode::W, Board::Left, PushDir::Up);
            shit!(VirtualKeyCode::A, Board::Left, PushDir::Left);
            shit!(VirtualKeyCode::S, Board::Left, PushDir::Down);
            shit!(VirtualKeyCode::D, Board::Left, PushDir::Right);

            shit!(VirtualKeyCode::I, Board::Right, PushDir::Up);
            shit!(VirtualKeyCode::J, Board::Right, PushDir::Left);
            shit!(VirtualKeyCode::K, Board::Right, PushDir::Down);
            shit!(VirtualKeyCode::L, Board::Right, PushDir::Right);

            // failed a move
            if !self.animations.last_move_success {
                let func = |mut offsets: HashMap<_, _>, prog| {
                    offsets.insert(0, (0, 0));
                    offsets
                };
                self.animations.begin_transition(Box::new(func));
            }
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        let level = self.get_current_level();
        level.render(renderer, &self.animations);
    }
}
