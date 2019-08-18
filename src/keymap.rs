use std::collections::HashMap;

use crate::platform::KeyCode;

pub struct Keymap(HashMap<KeyCode, bool>);

impl Keymap {
    pub fn new() -> Self {
        Keymap(HashMap::new())
    }

    pub fn pressed(&mut self, code: KeyCode) {
        self.0.insert(code, true);
    }

    pub fn release(&mut self, code: KeyCode) {
        self.0.insert(code, false);
    }

    pub fn is_pressed(&self, code: KeyCode) -> bool {
        if let Some(true) = self.0.get(&code) {
            true
        } else {
            false
        }
    }
}
