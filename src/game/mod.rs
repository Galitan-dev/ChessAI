use gfx_core::{command::Buffer, Factory, Resources};
use piston_window::{Context, Graphics, RenderArgs, TextureContext, UpdateArgs};

use crate::draw::Drawable;
use crate::utils::WINDOW_SIZE;

pub use self::board::BoardOrientation;
pub use self::board::{Board, BOARD_HEIGHT, BOARD_WIDTH};
pub use self::piece::Piece;

mod board;
mod piece;

pub struct Game {
    board: Board,
    mouse_position: [f64; 2],
    window_size: [f64; 2],
}

impl Game {
    pub fn new(board_orientation: BoardOrientation) -> Self {
        Self {
            board: Board::new(board_orientation),
            mouse_position: [0., 0.],
            window_size: WINDOW_SIZE,
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

    pub fn resize(&mut self, size: [f64; 2]) {
        self.window_size = size;
    }

    pub fn update(&mut self, _args: &UpdateArgs) {}

    pub fn mouse_cursor(&mut self, pos: [f64; 2]) {
        self.mouse_position = pos
    }

    pub fn click(&mut self) {
        let [x, y] = self.mouse_position;
        let [w, h] = self.window_size;
        let col = (x / w * BOARD_WIDTH as f64).floor();
        let row = (y / h * BOARD_HEIGHT as f64).floor();
        self.board.select(col as usize, row as usize);
    }
}
