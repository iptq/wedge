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
const LEVEL_TUTORIAL2: &str = include_str!("../levels/tutorial2.json");

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

        let levels = vec![
            Level::from_json(&LEVEL_TUTORIAL),
            Level::from_json(&LEVEL_TUTORIAL2),
        ];
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
        if let Event::WindowEvent { event, .. } = event {
            match event {
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
            }
        }
    }

    pub fn create_renderer<'b>(&self, target: &'b mut Frame) -> Renderer<'b, '_> {
        Renderer::new(self, target)
    }

    pub fn get_current_level(&self) -> &Level {
        self.levels.get(self.current_level).unwrap()
    }

    pub fn get_current_level_mut(&mut self) -> &mut Level {
        self.levels.get_mut(self.current_level).unwrap()
    }

    pub fn is_pressed(&self, code: VirtualKeyCode) -> bool {
        if let Some(true) = self.keymap.get(&code) {
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, delta: Duration) {
        macro_rules! shit {
            ($key:expr, $board:expr, $direction:expr) => {
                if self.is_pressed($key) {
                    println!("pushed: {:?}", $key);
                    let level = self.get_current_level_mut();
                    let result = level.try_move($board, $direction);
                    self.animations.begin_move_transition(result);
                    self.keymap.insert($key, false);
                }
            };
        }

        if self.animations.is_animating {
            // println!("animating. {:?}", self.animations.progress);
            self.animations.make_progress(delta);

            // we just finished!
            if !self.animations.is_animating {
                // apply the changes to the entities
                // this indirection is used to dodge a concurrent borrow
                let change_set = if let Some(Ok(change_set)) = &self.animations.last_move_result {
                    Some(change_set.clone())
                } else {
                    None
                };
                if let Some(change_set) = change_set {
                    let level = self.get_current_level_mut();
                    level.apply_change_set(change_set.clone());
                    self.check_win_condition();
                }
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
        }
    }

    fn check_win_condition(&mut self) {
        let level = self.get_current_level();
        if level.check_win_condition() {
            // go on to the next level
            self.current_level += 1;
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        let level = self.get_current_level();
        level.render(renderer, &self.animations);
    }
}
