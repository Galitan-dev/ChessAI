use gfx_core::{command::Buffer, Factory, Resources};
use piston_window::{Context, Graphics, RenderArgs, TextureContext, UpdateArgs};

use crate::draw::Drawable;

pub use self::board::BoardOrientation;
pub use self::board::{Board, BOARD_HEIGHT, BOARD_WIDTH};
pub use self::piece::Piece;

mod board;
mod piece;

pub struct Game {
    board: Board,
}

impl Game {
    pub fn new(board_orientation: BoardOrientation) -> Self {
        Self {
            board: Board::new(board_orientation),
        }
    }

    pub fn render<
        G: Graphics<Texture = piston_window::Texture<R>>,
        F: Factory<R>,
        R: Resources,
        C: Buffer<R>,
    >(
        &mut self,
        c: Context,
        g: &mut G,
        args: &RenderArgs,
        tc: &mut TextureContext<F, R, C>,
    ) {
        self.board.draw(c, g, args.window_size, tc);
    }

    pub fn update(&mut self, _args: &UpdateArgs) {}
}
