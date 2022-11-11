pub const BOARD_WIDTH: usize = 8;
pub const BOARD_HEIGHT: usize = 8;

pub struct Board {
    pub(super) width: usize,
    pub(super) height: usize,
}

impl Board {
    pub fn new() -> Self {
        Self {
            width: BOARD_WIDTH,
            height: BOARD_HEIGHT,
        }
    }
}
