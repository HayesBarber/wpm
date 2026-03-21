use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};

#[repr(C)]
struct WinSize {
    ws_row: libc::c_ushort,
    ws_col: libc::c_ushort,
    ws_xpixel: libc::c_ushort,
    ws_ypixel: libc::c_ushort,
}

static RUNNING: AtomicBool = AtomicBool::new(true);

fn enter_alternate_buffer() {
    print!("\x1b[?1049h");
    io::stdout().flush().unwrap();
}

fn leave_alternate_buffer() {
    print!("\x1b[?1049l");
    io::stdout().flush().unwrap();
}

fn clear_screen() {
    print!("\x1b[2J");
    io::stdout().flush().unwrap();
}

fn move_cursor(row: u16, col: u16) {
    print!("\x1b[{};{}H", row, col);
    io::stdout().flush().unwrap();
}

fn get_terminal_size() -> (u16, u16) {
    let mut ws = WinSize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    unsafe {
        libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut ws);
    }
    (ws.ws_col, ws.ws_row)
}

fn render_centered(text: &str) {
    let (cols, rows) = get_terminal_size();
    let row = rows / 2;
    let col = cols.saturating_sub(text.len() as u16) / 2;
    clear_screen();
    move_cursor(row, col);
    print!("{}", text);
    move_cursor(row, col);
    io::stdout().flush().unwrap();
}

pub fn run() {
    ctrlc::set_handler(|| {
        RUNNING.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    enter_alternate_buffer();

    let words = crate::generator::generate(25);
    render_centered(&words);

    while RUNNING.load(Ordering::SeqCst) {}

    leave_alternate_buffer();
}
