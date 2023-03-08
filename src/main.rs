// use std::default::default;

use data::TodoState;
use druid::{
    theme::{BUTTON_DARK, BUTTON_LIGHT},
    AppLauncher, Color, WindowDesc,
};
use im::Vector;
use saver::read_stored;
use ui::ui_builder;

mod data;
mod saver;
mod ui;
fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title("My app")
        .window_size((400., 400.));
    let stored = read_stored();
    let default_state = TodoState {
        todos: Vector::from(stored.tasks),
        ..Default::default()
    };

    AppLauncher::with_window(main_window)
        .configure_env(|env, _state| {
            env.set(BUTTON_DARK, Color::rgba8(100, 100, 120, 0));
            env.set(BUTTON_LIGHT, Color::rgba8(100, 100, 120, 100));
        })
        .launch(default_state)
        .expect("Failed to start")
}
