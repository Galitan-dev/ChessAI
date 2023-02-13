use anyhow::{anyhow, Result};
use piston_window::OpenGL;
use piston_window::{PistonWindow, WindowSettings};

pub const SIDE: f64 = 800.;

pub fn window() -> Result<PistonWindow> {
    let opengl = OpenGL::V3_2;

    let window = WindowSettings::new("chess-ai", [SIDE; 2])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .decorated(false)
        .automatic_close(true)
        .transparent(true)
        .build::<PistonWindow>()
        .map_err(|err| anyhow!(err.to_string()))?;

    Ok(window)
}
