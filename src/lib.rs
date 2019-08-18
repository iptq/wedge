#[macro_use]
extern crate glium;
extern crate nalgebra_glm as glm;
#[macro_use]
extern crate serde_derive;

mod animations;
mod color;
mod data;
mod enums;
mod game;
mod keymap;
mod level;
mod platform;
mod renderer;
mod resources;
mod screens;

use std::time::Instant;

use crate::game::Game;
use crate::platform::Event;

const GAME_WIDTH: u32 = 1024;
const GAME_HEIGHT: u32 = 768;

pub fn run() {
    let mut closed = false;
    let mut prev = Instant::now();

    while !closed {
        let now = Instant::now();
        let delta = now - prev;
    }
}

fn main() {
    let ctx = platform::create_window();
    let mut game = Game::new(&ctx);

    let mut closed = false;
    let mut prev = Instant::now();
    while !closed {
        let now = Instant::now();
        let delta = now - prev;

        platform::events(&ctx, |event| match event {
            Event::WindowClosed => closed = true,
            _ => game.handle_event(event),
        });

        game.update(delta);

        let mut target = display.draw();
        target.clear(None, Some((0.0, 0.0, 0.0, 1.0)), true, None, None);
        let mut renderer = game.create_renderer(&mut target);
        game.render(&mut renderer);
        target.finish().unwrap();

        prev = now;
    }
}
