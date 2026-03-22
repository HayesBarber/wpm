mod app;
mod banner;
mod engine;
mod generator;
mod input;
mod render;
mod screen;
mod types;

fn print_help() {
    println!("Usage: wpm [word_count]");
    println!();
    println!("A terminal typing speed test.");
    println!();
    println!("Arguments:");
    println!("  [word_count]  Number of words to type (default: 25, max: 100)");
    println!();
    println!("Options:");
    println!("  -h, --help    Print this help message");
}

fn main() {
    let first_arg = std::env::args().nth(1);
    if first_arg.as_deref() == Some("-h") || first_arg.as_deref() == Some("--help") {
        print_help();
        return;
    }

    let word_count: usize = first_arg
        .and_then(|s| s.parse().ok())
        .unwrap_or(25)
        .min(100);

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
