use std::time::Instant;

use crate::banner::BANNER;
use crate::types::{CharState, Layout, MAX_LINE_WIDTH, PADDING, TestStats, TextArea, TypedChar};

const BANNER_TO_CONTROLS_GAP: u16 = 2;
const CONTROLS_TO_TEXT_GAP: u16 = 3;
const SHRINKWRAP_PADDING: u16 = 12;

fn make_banner_lines(
    banner_rows: &[&str],
    cols: u16,
    start_row: u16,
) -> Vec<Vec<(u16, u16, char)>> {
    let mut lines = Vec::new();
    let available_width = cols.saturating_sub(2 * PADDING);

    for (idx, row) in banner_rows.iter().enumerate() {
        let line_len = row.chars().count() as u16;
        let col_offset = PADDING + available_width.saturating_sub(line_len) / 2;
        let mut line = Vec::new();
        for (col_idx, ch) in row.chars().enumerate() {
            if ch != ' ' {
                line.push((start_row + idx as u16, col_offset + col_idx as u16, ch));
            }
        }
        lines.push(line);
    }
    lines
}

fn make_controls_lines(cols: u16, start_row: u16) -> Vec<Vec<(u16, u16, char)>> {
    let parts = vec![
        "Tab:".to_string(),
        " restart ".to_string(),
        "Ctrl+C/Esc:".to_string(),
        " quit ".to_string(),
    ];
    let full_text: String = parts.concat();
    let text_len = full_text.chars().count() as u16;
    let available_width = cols.saturating_sub(2 * PADDING);
    let start_col = PADDING + available_width.saturating_sub(text_len) / 2;

    let mut result = Vec::new();
    let mut col = start_col;
    for part in &parts {
        let mut chars = Vec::new();
        for ch in part.chars() {
            chars.push((start_row, col, ch));
            col += 1;
        }
        result.push(chars);
    }
    result
}

pub fn layout(cols: u16, rows: u16, chars: &[TypedChar]) -> Layout {
    let max_width = std::cmp::min(cols.saturating_sub(2 * PADDING), MAX_LINE_WIDTH);
    let max_height = rows.saturating_sub(2 * PADDING);

    let mut lines: Vec<Vec<TypedChar>> = Vec::new();
    let mut line_lengths: Vec<u16> = Vec::new();
    let mut current_line: Vec<TypedChar> = Vec::new();
    let mut word_len: u16 = 0;
    let mut line_char_count: u16 = 0;

    for &tc in chars {
        if tc.ch == ' ' {
            if line_char_count > 0 && line_char_count + 1 + word_len > max_width {
                lines.push(current_line);
                line_lengths.push(line_char_count);
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
            line_lengths.push(line_char_count);
            current_line = Vec::new();
            line_char_count = 0;
        }
        line_char_count += word_len;
    }

    if !current_line.is_empty() {
        lines.push(current_line);
        line_lengths.push(line_char_count);
    }

    let banner_rows: Vec<&str> = BANNER.split('\n').collect();
    let banner_height: u16 = banner_rows.len() as u16;
    let controls_height: u16 = 1;
    let total_height = banner_height
        + BANNER_TO_CONTROLS_GAP
        + controls_height
        + CONTROLS_TO_TEXT_GAP
        + lines.len() as u16;
    let combined_start = PADDING + max_height.saturating_sub(total_height) / 2;
    let controls_start = combined_start + banner_height + BANNER_TO_CONTROLS_GAP;
    let text_start = controls_start + controls_height + CONTROLS_TO_TEXT_GAP;

    let banner_lines = make_banner_lines(&banner_rows, cols, combined_start);
    let controls_lines = make_controls_lines(cols, controls_start);

    let mut positioned_lines: Vec<Vec<(u16, u16, TypedChar)>> = Vec::new();
    for (line_idx, line) in lines.iter().enumerate() {
        let row = text_start + line_idx as u16;
        let line_len: u16 = line.iter().map(|tc| tc.ch.len_utf16() as u16).sum();
        let available_width = cols.saturating_sub(2 * PADDING);
        let start_col = PADDING + available_width.saturating_sub(line_len) / 2;
        let positioned: Vec<(u16, u16, TypedChar)> = line
            .iter()
            .enumerate()
            .map(|(col_idx, &tc)| (row, start_col + col_idx as u16, tc))
            .collect();
        positioned_lines.push(positioned);
    }

    let text_row_start = text_start.saturating_sub(3);
    let text_row_end = text_start + 3 + std::cmp::max(lines.len() as u16, 1);
    let max_line_len = line_lengths.iter().copied().max().unwrap_or(0);
    let text_area_width = max_line_len + 2 * SHRINKWRAP_PADDING;
    let available_width = cols.saturating_sub(2 * PADDING);
    let text_area_left = PADDING + available_width.saturating_sub(text_area_width) / 2;
    let text_col_start = text_area_left.saturating_sub(1);
    let text_col_end = text_area_left + text_area_width + 1;

    let mut border_lines = Vec::new();
    let border_top = text_row_start;
    let border_bottom = text_row_end.saturating_sub(1);
    let border_left = text_col_start;
    let border_right = text_col_end.saturating_sub(1);

    // Top and bottom edges
    for c in border_left..=border_right {
        let ch = if c == border_left || c == border_right {
            '+'
        } else {
            '─'
        };
        border_lines.push((
            border_top,
            c,
            TypedChar {
                ch,
                state: CharState::Border,
            },
        ));
        border_lines.push((
            border_bottom,
            c,
            TypedChar {
                ch,
                state: CharState::Border,
            },
        ));
    }
    // Left and right edges (excluding corners already drawn)
    for r in (border_top + 1)..border_bottom {
        border_lines.push((
            r,
            border_left,
            TypedChar {
                ch: '│',
                state: CharState::Border,
            },
        ));
        border_lines.push((
            r,
            border_right,
            TypedChar {
                ch: '│',
                state: CharState::Border,
            },
        ));
    }

    let mut cursor_row = text_start;
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
        banner_lines,
        controls_lines,
        border_lines,
        lines: positioned_lines,
        text_area: TextArea {
            row_start: text_row_start,
            row_end: text_row_end,
            col_start: text_col_start,
            col_end: text_col_end,
        },
        cursor_row,
        cursor_col,
    }
}

pub fn compute_stats(chars: &[TypedChar], start_time: Instant) -> TestStats {
    let elapsed_secs = start_time.elapsed().as_secs_f64();
    let total = chars.len();
    let correct = chars
        .iter()
        .filter(|tc| tc.state == CharState::Correct)
        .count();
    let errors = chars
        .iter()
        .filter(|tc| tc.state == CharState::Incorrect)
        .count();
    let accuracy = if total > 0 {
        (correct as f64 / total as f64) * 100.0
    } else {
        0.0
    };
    let wpm = if elapsed_secs > 0.0 {
        (correct as f64 * 60.0) / (elapsed_secs * 5.0)
    } else {
        0.0
    };
    TestStats {
        wpm,
        accuracy,
        errors,
        correct,
        total,
        elapsed_secs,
    }
}
