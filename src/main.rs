use opengl_graphics::OpenGL;
use piston::{EventSettings, Events, RenderEvent, UpdateEvent};

use crate::{game::Game, utils::create_window};

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod game;
mod utils;

pub const OPEN_GL: OpenGL = OpenGL::V4_5;

fn main() {
    let mut window = create_window();

    let mut game = Game::new();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }
    }
}
