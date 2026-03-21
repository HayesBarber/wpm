mod app;
mod engine;
mod generator;
mod input;
mod render;
mod types;

fn main() {
    render::setup();
    crate::input::enable_raw_mode().expect("Failed to enable raw mode");

    app::run();

    crate::input::disable_raw_mode().expect("Failed to disable raw mode");
    render::teardown();
}
