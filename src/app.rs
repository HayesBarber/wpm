use std::time::Instant;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use crate::screen::ScreenBuf;
use crate::types::{CharState, Layout, TestStats, TypedChar};

pub struct App {
    chars: Vec<TypedChar>,
    num_words: usize,
    layout: Layout,
    prev_buf: ScreenBuf,
    cursor_index: usize,
    term_rows: u16,
    term_cols: u16,
    start_time: Option<Instant>,
    prev_counter_row: u16,
    prev_counter_end_col: u16,
}

impl App {
    pub fn new(num_words: usize) -> Self {
        let words = crate::generator::generate(num_words);
        let chars: Vec<TypedChar> = words
            .chars()
            .map(|ch| TypedChar {
                ch,
                state: CharState::Pending,
            })
            .collect();

        let (term_cols, term_rows) = crate::render::get_terminal_size();
        let layout = crate::engine::layout(term_cols, term_rows, &chars, 0);

        crate::render::render_layout(&layout);

        let mut prev_buf = ScreenBuf::new(term_rows as usize, term_cols as usize);
        prev_buf.apply_layout(&layout);

        let prev_counter_row = layout.counter_line.first().map_or(0, |&(r, _, _)| r);
        let prev_counter_end_col = layout.counter_line.last().map_or(0, |&(_, c, _)| c + 1);

        App {
            chars,
            num_words,
            layout,
            prev_buf,
            cursor_index: 0,
            term_rows,
            term_cols,
            start_time: None,
            prev_counter_row,
            prev_counter_end_col,
        }
    }

    pub fn handle_event(&mut self, event: Event) -> bool {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers,
                ..
            }) if modifiers.contains(KeyModifiers::CONTROL) => true,
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => true,
            Event::Key(KeyEvent {
                code: KeyCode::Tab, ..
            }) => {
                self.reset();
                false
            }
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                ..
            }) => self.handle_backspace(),
            Event::Key(KeyEvent {
                code: KeyCode::Char(ch),
                modifiers,
                ..
            }) if !modifiers.contains(KeyModifiers::CONTROL) => self.handle_char_input(ch),
            _ => false,
        }
    }

    fn reset(&mut self) {
        let words = crate::generator::generate(self.num_words);
        self.chars = words
            .chars()
            .map(|ch| TypedChar {
                ch,
                state: CharState::Pending,
            })
            .collect();
        self.cursor_index = 0;
        self.start_time = None;
        self.refresh();
    }

    fn handle_char_input(&mut self, ch: char) -> bool {
        if self.cursor_index >= self.chars.len() {
            return true;
        }

        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }

        let expected = self.chars[self.cursor_index].ch;
        self.chars[self.cursor_index].state = if ch == expected {
            CharState::Correct
        } else {
            CharState::Incorrect
        };
        self.cursor_index += 1;
        self.refresh();

        self.cursor_index >= self.chars.len()
    }

    fn handle_backspace(&mut self) -> bool {
        if self.cursor_index == 0 {
            return false;
        }
        self.cursor_index -= 1;
        self.chars[self.cursor_index].state = CharState::Pending;
        self.refresh();
        false
    }

    pub fn stats(&self) -> Option<TestStats> {
        self.start_time
            .map(|start| crate::engine::compute_stats(&self.chars, start))
    }

    fn refresh(&mut self) {
        self.layout = crate::engine::layout(
            self.term_cols,
            self.term_rows,
            &self.chars,
            self.cursor_index,
        );

        let mut desired = ScreenBuf::new(self.prev_buf.rows, self.prev_buf.cols);
        desired.apply_layout(&self.layout);

        let new_counter_row = self.layout.counter_line.first().map_or(0, |&(r, _, _)| r);
        let new_counter_start_col = self.layout.counter_line.first().map_or(0, |&(_, c, _)| c);
        let new_counter_end_col = self
            .layout
            .counter_line
            .last()
            .map_or(0, |&(_, c, _)| c + 1);

        let clear_start = new_counter_start_col.min(
            self.prev_counter_end_col
                .saturating_sub(new_counter_end_col.saturating_sub(new_counter_start_col)),
        );
        let clear_end = new_counter_end_col.max(self.prev_counter_end_col);

        self.prev_counter_row = new_counter_row;
        self.prev_counter_end_col = clear_end;

        let changes = desired.diff(&self.prev_buf);
        crate::render::render_changes(
            &changes,
            self.layout.cursor_row,
            self.layout.cursor_col,
            &self.layout.counter_line,
            new_counter_row,
            clear_start,
            clear_end,
        );

        self.prev_buf = desired;
    }
}
