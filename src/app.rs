use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use crate::types::{CharState, Layout, TypedChar};

pub struct App {
    words: String,
    chars: Vec<TypedChar>,
    layout: Layout,
}

impl App {
    pub fn init() -> Self {
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

        App { words, chars, layout }
    }

    pub fn handle_event(&mut self, event: Event) -> bool {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers,
                ..
            }) if modifiers.contains(KeyModifiers::CONTROL) => true,
            _ => false,
        }
    }
}
