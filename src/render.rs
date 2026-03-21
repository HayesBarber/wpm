use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};

#[repr(C)]
struct WinSize {
    ws_row: libc::c_ushort,
    ws_col: libc::c_ushort,
    ws_xpixel: libc::c_ushort,
    ws_ypixel: libc::c_ushort,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CharState {
    Pending,
    Correct,
    Incorrect,
}

#[derive(Clone, Copy)]
pub struct TypedChar {
    pub ch: char,
    pub state: CharState,
}

struct Layout {
    lines: Vec<Vec<(u16, u16, TypedChar)>>,
    cursor_row: u16,
    cursor_col: u16,
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

fn layout(cols: u16, rows: u16, chars: &[TypedChar]) -> Layout {
    let max_width = cols.saturating_sub(2 * PADDING);
    let max_height = rows.saturating_sub(2 * PADDING);

    let mut lines: Vec<Vec<TypedChar>> = Vec::new();
    let mut current_line: Vec<TypedChar> = Vec::new();
    let mut word_len: u16 = 0;
    let mut line_char_count: u16 = 0;

    for &tc in chars {
        if tc.ch == ' ' {
            if line_char_count > 0 && line_char_count + 1 + word_len > max_width {
                lines.push(current_line);
                current_line = Vec::new();
                line_char_count = 0;
            }
            if line_char_count > 0 {
                line_char_count += 1;
            }
            current_line.push(tc);
            line_char_count += word_len;
            word_len = 0;
        } else {
            word_len += 1;
            current_line.push(tc);
        }
    }

    if word_len > 0 {
        if line_char_count > 0 && line_char_count + 1 + word_len > max_width {
            lines.push(current_line);
            current_line = Vec::new();
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    let start_row = PADDING + max_height.saturating_sub(lines.len() as u16) / 2;

    let mut positioned_lines: Vec<Vec<(u16, u16, TypedChar)>> = Vec::new();
    for (line_idx, line) in lines.iter().enumerate() {
        let row = start_row + line_idx as u16;
        let line_len: u16 = line.iter().map(|tc| tc.ch.len_utf16() as u16).sum();
        let start_col = PADDING + max_width.saturating_sub(line_len) / 2;
        let positioned: Vec<(u16, u16, TypedChar)> = line
            .iter()
            .enumerate()
            .map(|(col_idx, &tc)| (row, start_col + col_idx as u16, tc))
            .collect();
        positioned_lines.push(positioned);
    }

    let mut cursor_row = start_row;
    let mut cursor_col = PADDING;
    let mut found = false;
    'outer: for line in &positioned_lines {
        for &(row, col, tc) in line {
            if tc.state == CharState::Pending {
                cursor_row = row;
                cursor_col = col;
                found = true;
                break 'outer;
            }
        }
    }
    if !found && !positioned_lines.is_empty() && !positioned_lines[0].is_empty() {
        cursor_row = positioned_lines[0][0].0;
        cursor_col = positioned_lines[0][0].1;
    }

    Layout {
        lines: positioned_lines,
        cursor_row,
        cursor_col,
    }
}

fn render_layout(layout: &Layout) {
    clear_screen();
    for line in &layout.lines {
        for &(row, col, tc) in line {
            move_cursor(row, col);
            match tc.state {
                CharState::Correct => print!("\x1b[32m{}\x1b[0m", tc.ch),
                CharState::Incorrect => print!("\x1b[31m{}\x1b[0m", tc.ch),
                CharState::Pending => print!("{}", tc.ch),
            }
            io::stdout().flush().unwrap();
        }
    }
    move_cursor(layout.cursor_row, layout.cursor_col);
    io::stdout().flush().unwrap();
}

pub fn run() {
    ctrlc::set_handler(|| {
        RUNNING.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    enter_alternate_buffer();

    let words = crate::generator::generate(25);
    let chars: Vec<TypedChar> = words
        .chars()
        .map(|ch| TypedChar {
            ch,
            state: CharState::Pending,
        })
        .collect();

    let (cols, rows) = get_terminal_size();
    let l = layout(cols, rows, &chars);
    render_layout(&l);

    while RUNNING.load(Ordering::SeqCst) {}

    leave_alternate_buffer();
}
