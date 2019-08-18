mod editor;
mod menu;
mod play;

use std::sync::Arc;
use std::time::Duration;

use crate::keymap::Keymap;
use crate::renderer::Renderer;

pub use self::editor::EditorScreen;
pub use self::menu::MenuScreen;
pub use self::play::PlayScreen;

pub trait Screen {
    fn update(&mut self, delta: Duration, keymap: &Keymap) -> ScreenAction {
        ScreenAction::None
    }

    fn render(&self, renderer: &mut Renderer) {}
}

pub enum ScreenAction {
    None,
    Push(Box<dyn Screen>),
    Pop(usize),
}

pub struct ScreenStack(Vec<Box<dyn Screen>>);

impl ScreenStack {
    pub fn with<S: 'static + Screen>(screen: S) -> Self {
        let mut stack = Vec::<Box<Screen>>::new();
        stack.push(Box::new(screen));
        ScreenStack(stack)
    }

    pub fn top(&self) -> impl AsRef<dyn Screen + 'static> + '_ {
        self.0.last().unwrap()
    }

    pub fn top_mut(&mut self) -> impl AsMut<dyn Screen + 'static> + '_ {
        self.0.last_mut().unwrap()
    }

    pub fn update(&mut self, delta: Duration, keymap: &Keymap) {
        let result = {
            let mut screen = self.top_mut();
            let screen = screen.as_mut();
            screen.update(delta, keymap)
        };

        match result {
            ScreenAction::None => (),
            ScreenAction::Push(new_screen) => {
                self.0.push(new_screen);
            }
            ScreenAction::Pop(n) => {
                self.0.truncate(self.0.len() - 1);
            }
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        let screen = self.top();
        let screen = screen.as_ref();
        screen.render(renderer)
    }
}
