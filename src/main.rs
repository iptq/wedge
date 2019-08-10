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
mod level;
mod renderer;
mod resources;

use std::time::Instant;

use glium::glutin::dpi::PhysicalSize;
use glium::glutin::{ContextBuilder, Event, EventsLoop, WindowBuilder, WindowEvent};
use glium::{Display, Surface};

use crate::game::Game;

const GAME_WIDTH: u32 = 1024;
const GAME_HEIGHT: u32 = 768;

fn main() {
    let mut events_loop = EventsLoop::new();
    let primary_monitor = events_loop.get_primary_monitor();
    let dpi_factor = primary_monitor.get_hidpi_factor();
    let dimensions: PhysicalSize = (GAME_WIDTH, GAME_HEIGHT).into();

    let wb = WindowBuilder::new()
        .with_dimensions(dimensions.to_logical(dpi_factor))
        .with_resizable(false)
        .with_title("wedge");
    let cb = ContextBuilder::new();
    let display = Display::new(wb, cb, &events_loop).unwrap();

    {
        let gl_window = display.gl_window();
        let window = gl_window.window();
        println!("size: {:?}", window.get_inner_size());
    }

    let mut game = Game::new(&display);

    let mut closed = false;
    let mut prev = Instant::now();
    while !closed {
        let now = Instant::now();
        let delta = now - prev;

        events_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => closed = true,
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
