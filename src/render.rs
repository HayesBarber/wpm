use std::io::{self, Write};

use crate::types::{CharState, Layout};

#[repr(C)]
struct WinSize {
    ws_row: libc::c_ushort,
    ws_col: libc::c_ushort,
    ws_xpixel: libc::c_ushort,
    ws_ypixel: libc::c_ushort,
}

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

fn hide_cursor() {
    print!("\x1b[?25l");
}

fn show_cursor() {
    print!("\x1b[?25h");
}

fn move_cursor(row: u16, col: u16) {
    print!("\x1b[{};{}H", row, col);
}

pub fn get_terminal_size() -> (u16, u16) {
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

pub fn render_layout(layout: &Layout) {
    clear_screen();
    hide_cursor();
    for line in &layout.lines {
        for &(row, col, tc) in line {
            move_cursor(row, col);
            match tc.state {
                CharState::Correct => print!("\x1b[32m{}\x1b[0m", tc.ch),
                CharState::Incorrect => print!("\x1b[31m{}\x1b[0m", tc.ch),
                CharState::Pending => print!("\x1b[90m{}\x1b[0m", tc.ch),
            }
        }
    }
    move_cursor(layout.cursor_row, layout.cursor_col);
    show_cursor();
    io::stdout().flush().unwrap();
}

pub fn setup() {
    enter_alternate_buffer();
}

pub fn teardown() {
    leave_alternate_buffer();
}
