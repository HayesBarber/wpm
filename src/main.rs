mod app;
mod engine;
mod generator;
mod input;
mod render;
mod types;

fn main() {
    let word_count: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(25);

    render::setup();
    input::enable_raw_mode().expect("Failed to enable raw mode");

    let mut app = app::App::new(word_count);

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
