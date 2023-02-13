use anyhow::{anyhow, Result};
use piston_window::OpenGL;
use piston_window::{PistonWindow, WindowSettings};

pub const SIZE: [f64; 2] = [800.; 2];

pub fn window() -> Result<PistonWindow> {
    let opengl = OpenGL::V3_2;

    let window = WindowSettings::new("chess-ai", SIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .automatic_close(true)
        .build::<PistonWindow>()
        .map_err(|err| anyhow!(err.to_string()))?;

    Ok(window)
}
