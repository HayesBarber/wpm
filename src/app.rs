use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use crate::types::{CharState, Layout, TypedChar};

pub struct App {
    words: String,
    chars: Vec<TypedChar>,
    layout: Layout,
    cursor_index: usize,
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

        let (cols, rows) = crate::render::get_terminal_size();
        let layout = crate::engine::layout(cols, rows, &chars);
        crate::render::render_layout(&layout);

        App {
            words,
            chars,
            layout,
            cursor_index: 0,
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
                if self.cursor_index < self.chars.len() {
                    let expected = self.chars[self.cursor_index].ch;
                    self.chars[self.cursor_index].state = if ch == expected {
                        CharState::Correct
                    } else {
                        CharState::Incorrect
                    };
                    self.cursor_index += 1;

                    let (cols, rows) = crate::render::get_terminal_size();
                    self.layout = crate::engine::layout(cols, rows, &self.chars);
                    crate::render::render_layout(&self.layout);
                }
                false
            }
            _ => false,
        }
    }
}
