use std::time::Instant;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use crate::types::{CharState, Layout, TestStats, TypedChar};

pub struct App {
    chars: Vec<TypedChar>,
    layout: Layout,
    cursor_index: usize,
    term_rows: u16,
    term_cols: u16,
    start_time: Option<Instant>,
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
        let layout = crate::engine::layout(term_cols, term_rows, &chars);
        crate::render::render_layout(&layout);

        App {
            chars,
            layout,
            cursor_index: 0,
            term_rows,
            term_cols,
            start_time: None,
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
        self.layout = crate::engine::layout(self.term_cols, self.term_rows, &self.chars);
        crate::render::render_layout(&self.layout);
    }
}
