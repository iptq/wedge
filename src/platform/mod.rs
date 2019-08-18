pub enum Event {
    Keyboard(KeyCode, KeyState),
    WindowClosed,
    WindowResized(u32, u32),
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum KeyCode {
    Space,
    Escape,
    E,
    W,
    A,
    S,
    D,
    I,
    J,
    K,
    L,
}

pub enum KeyState {
    Pressed,
    Released,
}

#[cfg(not(target = "wasm32-unknown-unknown"))]
mod desktop;
#[cfg(not(target = "wasm32-unknown-unknown"))]
pub use self::desktop::*;

#[cfg(target = "wasm32-unknown-unknown")]
mod web;
#[cfg(target = "wasm32-unknown-unknown")]
pub use self::web::*;
