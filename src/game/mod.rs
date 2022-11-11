use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};

use crate::{game::draw::Drawable, OPEN_GL};

use self::board::Board;

mod board;
mod draw;

pub struct Game {
    gl: GlGraphics,
    board: Board,
}

impl Game {
    pub fn new() -> Self {
        Self {
            gl: GlGraphics::new(OPEN_GL),
            board: Board::new(),
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |c, gl| {
            self.board.draw(c, gl, args.window_size);
        });
    }

    pub fn update(&mut self, _args: &UpdateArgs) {}
}
