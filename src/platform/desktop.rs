use glium::glutin::dpi::PhysicalSize;
use glium::glutin::{
    ContextBuilder, ElementState, Event as GlutinEvent, EventsLoop, VirtualKeyCode, WindowBuilder,
    WindowEvent,
};
use glium::{Display, Program, ProgramCreationError};

use crate::platform::{Event, KeyCode, KeyState};

pub type WindowContext = (Display, EventsLoop);

pub type GlslCompileContext<'a> = &'a Display;
pub type GlslProgram = Program;
pub type GlslCompileError = ProgramCreationError;

pub fn create_window() -> WindowContext {
    let mut events_loop = EventsLoop::new();
    let primary_monitor = events_loop.get_primary_monitor();
    let dpi_factor = primary_monitor.get_hidpi_factor();
    let dimensions: PhysicalSize = (crate::GAME_WIDTH, crate::GAME_HEIGHT).into();

    let wb = WindowBuilder::new()
        .with_dimensions(dimensions.to_logical(dpi_factor))
        .with_resizable(false)
        .with_title("wedge");
    let cb = ContextBuilder::new();
    let display = Display::new(wb, cb, &events_loop).unwrap();
    (display, events_loop)
}

pub fn get_gl_context(window: &WindowContext) -> GlslCompileContext {
    &window.0
}

fn translate_event(event: GlutinEvent) -> Option<Event> {
    match event {
        GlutinEvent::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => Some(Event::WindowClosed),
            WindowEvent::Resized(size) => {
                let pair: (u32, u32) = size.into();
                Some(Event::WindowResized(pair.0, pair.1))
            }
        },
        _ => None,
    }
}

fn translate_key_code(key_code: VirtualKeyCode) -> Option<KeyCode> {
    match key_code {
        VirtualKeyCode::Space => Some(KeyCode::Space),
        VirtualKeyCode::Escape => Some(KeyCode::Escape),
        VirtualKeyCode::E => Some(KeyCode::E),
        VirtualKeyCode::W => Some(KeyCode::W),
        VirtualKeyCode::A => Some(KeyCode::A),
        VirtualKeyCode::S => Some(KeyCode::S),
        VirtualKeyCode::D => Some(KeyCode::D),
        VirtualKeyCode::I => Some(KeyCode::I),
        VirtualKeyCode::J => Some(KeyCode::J),
        VirtualKeyCode::K => Some(KeyCode::K),
        VirtualKeyCode::L => Some(KeyCode::L),
        _ => None,
    }
}

fn translate_key_state(key_state: ElementState) -> KeyState {
    match key_state {
        ElementState::Pressed => KeyState::Pressed,
        ElementState::Released => KeyState::Released,
    }
}

pub fn events<F>(ctx: &WindowContext, f: F)
where
    F: Fn(Event),
{
    ctx.1.poll_events(|event| {
        if let Some(translated) = translate_event(event) {
            f(translated);
        }
    });
}

pub fn compile_glsl_program<'a>(
    context: &'a Display,
    vert: impl AsRef<str>,
    frag: impl AsRef<str>,
) -> Result<Program, ProgramCreationError> {
    let vert = vert.as_ref();
    let frag = frag.as_ref();

    Program::from_source(context, vert, frag, None)
}
