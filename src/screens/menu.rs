use std::time::Duration;

use glium::glutin::VirtualKeyCode;

use crate::keymap::Keymap;
use crate::screens::{PlayScreen, Screen, ScreenAction};

pub struct MenuScreen;

impl Screen for MenuScreen {
    fn update(&mut self, delta: Duration, keymap: &Keymap) -> ScreenAction {
        if keymap.is_pressed(VirtualKeyCode::Space) {
            let play_screen = PlayScreen::new();
            ScreenAction::Push(Box::new(play_screen))
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
