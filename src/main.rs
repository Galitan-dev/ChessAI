use anyhow::Result;
use board::Board;
use piston_window::{EventSettings, Events, Graphics, RenderArgs, RenderEvent, UpdateEvent};
use render::Render;
use window::window;

extern crate anyhow;
extern crate piston_window;

mod board;
mod render;
mod window;

fn main() -> Result<()> {
    let mut window = window()?;

    let mut board = Board::new();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            window.draw_2d(&e, |c, g, _| {
                board.render(c, g);
            });
        }

        if let Some(args) = e.update_args() {}
    }

    Ok(())
}
