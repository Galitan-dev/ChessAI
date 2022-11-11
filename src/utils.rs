use glutin_window::GlutinWindow as Window;
use piston::window::WindowSettings;

use crate::OPEN_GL;

pub fn create_window() -> Window {
    WindowSettings::new("spinning-square", [600, 600])
        .graphics_api(OPEN_GL)
        .exit_on_esc(true)
        .resizable(false)
        .decorated(false)
        .build()
        .unwrap()
}
