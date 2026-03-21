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
const PADDING: u16 = 8;

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

fn wrap_words(text: &str, max_width: u16) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut current_line = String::new();
    for word in text.split_whitespace() {
        if !current_line.is_empty() && current_line.len() + 1 + word.len() > max_width as usize {
            lines.push(current_line);
            current_line = String::from(word);
        } else {
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    lines
}

fn render_centered(text: &str) {
    let (cols, rows) = get_terminal_size();
    let available_width = cols.saturating_sub(2 * PADDING);
    let available_height = rows.saturating_sub(2 * PADDING);
    let lines = wrap_words(text, available_width);
    let line_count = lines.len() as u16;
    let start_row = PADDING + available_height.saturating_sub(line_count) / 2;
    clear_screen();
    for (i, line) in lines.iter().enumerate() {
        let col = PADDING + available_width.saturating_sub(line.len() as u16) / 2;
        move_cursor(start_row + i as u16, col);
        print!("{}", line);
    }
    if lines.len() > 0 {
        move_cursor(
            start_row as u16,
            PADDING + available_width.saturating_sub(lines[0].len() as u16) / 2,
        );
    }
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
