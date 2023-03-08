use druid::{AppLauncher, WindowDesc};
use ui::ui_builder;

mod data;
mod ui;
fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title("My app")
        .window_size((400., 400.));
    AppLauncher::with_window(main_window)
        .launch(data::TodoState::default())
        .expect("Failed to start")
}
