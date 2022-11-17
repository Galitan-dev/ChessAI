use piston_window::{PistonWindow, WindowSettings};

use crate::OPEN_GL;

pub const WINDOW_SIZE: [f64; 2] = [600., 600.];

pub fn create_window() -> PistonWindow {
    WindowSettings::new("spinning-square", WINDOW_SIZE)
        .graphics_api(OPEN_GL)
        .exit_on_esc(true)
        .resizable(false)
        //.decorated(false)
        .build()
        .unwrap()
}
