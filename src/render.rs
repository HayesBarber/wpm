use std::io::{self, Write};

use crate::types::{
    COLOR_BG, COLOR_CORRECT, COLOR_INCORRECT, COLOR_PENDING, COLOR_RESET, CharState, Layout,
    TestStats, TextArea, TypedChar,
};

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

fn set_line_cursor() {
    print!("\x1b[6 q");
    io::stdout().flush().unwrap();
}

fn reset_cursor_style() {
    print!("\x1b[0 q");
    io::stdout().flush().unwrap();
}

fn move_cursor(row: u16, col: u16) {
    print!("\x1b[{};{}H", row, col);
}

fn print_styled_bg(ch: char, state: CharState) {
    match state {
        CharState::Background => print!("{} {}{}", COLOR_BG, ch, COLOR_RESET),
        CharState::Correct => print!("{}{}{}{}", COLOR_BG, COLOR_CORRECT, ch, COLOR_RESET),
        CharState::Incorrect => print!("{}{}{}{}", COLOR_BG, COLOR_INCORRECT, ch, COLOR_RESET),
        CharState::Pending => print!("{}{}{}{}", COLOR_BG, COLOR_PENDING, ch, COLOR_RESET),
    }
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

    for line in &layout.banner_lines {
        for &(row, col, ch) in line {
            move_cursor(row, col);
            print!("{}", ch);
        }
    }
    let TextArea {
        row_start,
        row_end,
        col_start,
        col_end,
    } = layout.text_area;
    for r in row_start..row_end {
        for c in col_start..col_end {
            move_cursor(r, c);
            print_styled_bg(' ', CharState::Background);
        }
    }
    for line in &layout.lines {
        for &(row, col, tc) in line {
            move_cursor(row, col);
            print_styled_bg(tc.ch, tc.state);
        }
    }
    move_cursor(layout.cursor_row, layout.cursor_col);
    show_cursor();
    io::stdout().flush().unwrap();
}

pub fn render_changes(changes: &[(u16, u16, TypedChar)], cursor_row: u16, cursor_col: u16) {
    if changes.is_empty() {
        return;
    }

    hide_cursor();
    for &(row, col, tc) in changes {
        move_cursor(row, col);
        print_styled_bg(tc.ch, tc.state);
    }
    move_cursor(cursor_row, cursor_col);
    show_cursor();
    io::stdout().flush().unwrap();
}

pub fn setup() {
    enter_alternate_buffer();
    set_line_cursor();
}

pub fn teardown() {
    reset_cursor_style();
    leave_alternate_buffer();
}

pub fn print_stats(stats: &TestStats) {
    println!("WPM:        {:.0}", stats.wpm);
    println!("Accuracy:   {:.1}%", stats.accuracy);
    println!("Errors:     {}", stats.errors);
    println!("Correct:    {}", stats.correct);
    println!("Total:      {}", stats.total);
    println!("Time:       {:.1}s", stats.elapsed_secs);
}
