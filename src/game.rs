use std::collections::HashMap;
use std::time::Duration;

use glium::glutin::{ElementState, Event, VirtualKeyCode, WindowEvent};
use glium::{Display, Frame};

use crate::animations::AnimationState;
use crate::enums::{Board, PushDir};
use crate::keymap::Keymap;
use crate::level::Level;
use crate::renderer::Renderer;
use crate::resources::Resources;
use crate::screens::{MenuScreen, Screen, ScreenStack};

const SEGMENT_VERT: &str = include_str!("../shaders/segment.vert");
const SEGMENT_FRAG: &str = include_str!("../shaders/segment.frag");
const CELL_VERT: &str = include_str!("../shaders/cell.vert");
const CELL_FRAG: &str = include_str!("../shaders/cell.frag");

const SEGMENT_IMAGE: &[u8] = include_bytes!("../textures/segment.png");

pub struct Game<'a> {
    pub resources: Resources,
    pub display: &'a Display,
    keymap: Keymap,
    screen_stack: ScreenStack,
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

        // bruh
        let screen_stack = ScreenStack::with(MenuScreen::new());

        Game {
            resources,
            display,
            keymap: Keymap::new(),
            screen_stack,
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::Resized(size) => self.resources.window_dimensions = size.into(),
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(code) = &input.virtual_keycode {
                        if let ElementState::Pressed = &input.state {
                            self.keymap.pressed(*code);
                        } else {
                            self.keymap.release(*code);
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

    pub fn update(&mut self, delta: Duration) {
        self.screen_stack.update(delta, &self.keymap);
    }

    pub fn render(&self, renderer: &mut Renderer) {
        self.screen_stack.render(renderer);
    }
}
