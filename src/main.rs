mod app;
mod engine;
mod generator;
mod input;
mod render;
mod types;

fn main() {
    render::setup();
    crate::input::enable_raw_mode().expect("Failed to enable raw mode");

    let mut app = app::App::new();

    loop {
        match crate::input::read_event() {
            Ok(event) => {
                if app.handle_event(event) {
                    break;
                }
            }
            _ => {}
        }
    }

    crate::input::disable_raw_mode().expect("Failed to disable raw mode");
    render::teardown();
}
