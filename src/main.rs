use game::BoardOrientation;
use piston_window::{EventSettings, Events, OpenGL, RenderEvent, UpdateEvent};

use crate::{game::Game, utils::create_window};

extern crate gfx_core;
extern crate piston_window;

pub mod draw;
pub mod game;
pub mod utils;

pub const OPEN_GL: OpenGL = OpenGL::V4_5;

fn main() {
    let mut window = create_window();
    let mut texture_context = window.create_texture_context();
    let mut game = Game::new(BoardOrientation::White);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            window.draw_2d(&e, |c, g, _| {
                game.render(c, g, &args, &mut texture_context);
            });
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }
    }
}
