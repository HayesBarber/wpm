mod app;
mod engine;
mod generator;
mod input;
mod render;
mod types;

fn main() {
    render::setup();
    input::enable_raw_mode().expect("Failed to enable raw mode");

    let mut app = app::App::new();

    loop {
        match input::read_event() {
            Ok(event) => {
                if app.handle_event(event) {
                    break;
                }
            }
            _ => {}
        }
    }

    let stats = app.stats();
    render::teardown();
    input::disable_raw_mode().expect("Failed to disable raw mode");

    if let Some(s) = stats {
        render::print_stats(&s);
    }
}
