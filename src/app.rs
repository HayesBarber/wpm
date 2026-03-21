use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use crate::types::{CharState, Layout, TypedChar};

pub struct App {
    chars: Vec<TypedChar>,
    layout: Layout,
    cursor_index: usize,
    term_rows: u16,
    term_cols: u16,
}

impl App {
    pub fn new() -> Self {
        let words = crate::generator::generate(25);
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
                code: KeyCode::Char(ch),
                modifiers,
                ..
            }) if !modifiers.contains(KeyModifiers::CONTROL) => {
                self.handle_char_input(ch);
                false
            }
            _ => false,
        }
    }

    fn handle_char_input(&mut self, ch: char) {
        if self.cursor_index >= self.chars.len() {
            return;
        }

        let expected = self.chars[self.cursor_index].ch;
        self.chars[self.cursor_index].state = if ch == expected {
            CharState::Correct
        } else {
            CharState::Incorrect
        };
        self.cursor_index += 1;
        self.refresh();
    }

    fn refresh(&mut self) {
        self.layout = crate::engine::layout(self.term_cols, self.term_rows, &self.chars);
        crate::render::render_layout(&self.layout);
    }
}
