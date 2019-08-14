use std::time::Duration;

use glium::glutin::VirtualKeyCode;

use crate::keymap::Keymap;
use crate::screens::{EditorScreen, PlayScreen, Screen, ScreenAction};

pub struct MenuScreen;

impl Screen for MenuScreen {
    fn update(&mut self, delta: Duration, keymap: &Keymap) -> ScreenAction {
        if keymap.is_pressed(VirtualKeyCode::Space) {
            let play_screen = PlayScreen::new();
            ScreenAction::Push(Box::new(play_screen))
        } else if keymap.is_pressed(VirtualKeyCode::E) {
            let editor_screen = EditorScreen::new();
            ScreenAction::Push(Box::new(editor_screen))
        } else {
            ScreenAction::None
        }
    }
}

impl MenuScreen {
    pub fn new() -> MenuScreen {
        MenuScreen
    }
}