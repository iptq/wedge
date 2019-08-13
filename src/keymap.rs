use std::collections::HashMap;

use glium::glutin::VirtualKeyCode;

pub struct Keymap(HashMap<VirtualKeyCode, bool>);

impl Keymap {
    pub fn new() -> Self {
        Keymap(HashMap::new())
    }

    pub fn pressed(&mut self, code: VirtualKeyCode) {
        self.0.insert(code, true);
    }

    pub fn release(&mut self, code: VirtualKeyCode) {
        self.0.insert(code, false);
    }

    pub fn is_pressed(&self, code: VirtualKeyCode) -> bool {
        if let Some(true) = self.0.get(&code) {
            true
        } else {
            false
        }
    }
}
