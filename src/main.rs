use anyhow::Result;
use board::Board;
use piston_window::{EventSettings, Events, RenderEvent, UpdateEvent};
use render::Render;
use window::window;

extern crate find_folder;
extern crate num;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate concat_arrays;
extern crate anyhow;
extern crate piston_window;

mod board;
mod piece;
mod render;
mod window;

fn main() -> Result<()> {
    let mut window = window()?;

    let board = Board::default();

    let mut texture_context = window.create_texture_context();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            window.draw_2d(&e, |c, g, _| {
                board.render(args, c, g, &mut texture_context);
            });
        }

        if let Some(_args) = e.update_args() {}
    }

    Ok(())
}
