use std::time::Duration;

use glium::glutin::VirtualKeyCode;

use crate::animations::AnimationState;
use crate::enums::{Board, PushDir};
use crate::keymap::Keymap;
use crate::level::Level;
use crate::renderer::Renderer;
use crate::screens::{Screen, ScreenAction};

const LEVEL_TUTORIAL: &str = include_str!("../../levels/tutorial.json");
const LEVEL_TUTORIAL2: &str = include_str!("../../levels/tutorial2.json");

pub struct PlayScreen {
    animations: AnimationState,
    levels: Vec<Level>,
    current_level: usize,
}

impl Screen for PlayScreen {
    fn update(&mut self, delta: Duration, keymap: &Keymap) -> ScreenAction {
        macro_rules! trigger_move {
            ($key:expr, $board:expr, $direction:expr) => {
                if keymap.is_pressed($key) {
                    let level = self.get_current_level_mut();
                    let result = level.try_move($board, $direction);
                    self.animations.begin_move_transition(result);
                }
            };
        }

        if self.animations.is_animating {
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
            trigger_move!(VirtualKeyCode::W, Board::Left, PushDir::Up);
            trigger_move!(VirtualKeyCode::A, Board::Left, PushDir::Left);
            trigger_move!(VirtualKeyCode::S, Board::Left, PushDir::Down);
            trigger_move!(VirtualKeyCode::D, Board::Left, PushDir::Right);

            trigger_move!(VirtualKeyCode::I, Board::Right, PushDir::Up);
            trigger_move!(VirtualKeyCode::J, Board::Right, PushDir::Left);
            trigger_move!(VirtualKeyCode::K, Board::Right, PushDir::Down);
            trigger_move!(VirtualKeyCode::L, Board::Right, PushDir::Right);
        }

        if keymap.is_pressed(VirtualKeyCode::Escape) {
            return ScreenAction::Pop(1);
        }

        ScreenAction::None
    }

    fn render(&self, renderer: &mut Renderer) {
        let level = self.get_current_level();
        level.render(renderer, &self.animations);
    }
}

impl PlayScreen {
    pub fn get_current_level(&self) -> &Level {
        self.levels.get(self.current_level).unwrap()
    }

    pub fn get_current_level_mut(&mut self) -> &mut Level {
        self.levels.get_mut(self.current_level).unwrap()
    }

    pub fn new() -> PlayScreen {
        let levels = vec![
            Level::from_json(&LEVEL_TUTORIAL),
            Level::from_json(&LEVEL_TUTORIAL2),
        ];

        PlayScreen {
            levels,
            current_level: 0,
            animations: AnimationState::new(),
        }
    }

    fn check_win_condition(&mut self) {
        let level = self.get_current_level();
        if level.check_win_condition() {
            // go on to the next level
            self.current_level += 1;
        }
    }
}
