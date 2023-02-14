use std::time::Duration;

use anyhow::Result;
use board::Board;
use piston_window::{
    Button, ButtonState, Event, EventSettings, Events, Input, Motion, MouseButton, RenderEvent,
    UpdateEvent,
};
use render::{piece::texture_bank, Render};
use window::window;

extern crate find_folder;
extern crate num;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate concat_arrays;
extern crate anyhow;
extern crate piston_window;
extern crate rand;
extern crate rodio;

mod board;
mod piece;
mod render;
mod window;

fn main() -> Result<()> {
    let mut window = window()?;

    let mut board = Board::default();

    let mut texture_context = window.create_texture_context();
    let texture_bank = texture_bank(&mut texture_context);

    let mut events = Events::new(EventSettings::new());
    let mut mouse_pos = [0.0; 2];
    let mut window_size = window::SIZE;
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            window.draw_2d(&e, |c, g, _| {
                board.render(args, c, g, &texture_bank, mouse_pos);
            });
        }

        if let Some(args) = e.update_args() {
            board.update(Duration::from_secs_f64(args.dt));
        }

        if let Event::Input(input, _) = e {
            match input {
                Input::Resize(args) => window_size = args.window_size,
                Input::Button(args) => {
                    if args.button == Button::Mouse(MouseButton::Left) {
                        match args.state {
                            ButtonState::Press => board.mouse_press(
                                mouse_pos[0] / window_size[0],
                                mouse_pos[1] / window_size[1],
                            ),
                            ButtonState::Release => board.mouse_relase(
                                mouse_pos[0] / window_size[0],
                                mouse_pos[1] / window_size[1],
                            ),
                        }
                    }
                }
                Input::Move(motion) => match motion {
                    Motion::MouseCursor(pos) => mouse_pos = pos,
                    _ => (),
                },
                _ => (),
            }
        }
    }

    Ok(())
}
