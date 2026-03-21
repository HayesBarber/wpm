use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use crate::types::{CharState, TypedChar};

pub fn run() {
    let words = crate::generator::generate(25);
    let chars: Vec<TypedChar> = words
        .chars()
        .map(|ch| TypedChar {
            ch,
            state: CharState::Pending,
        })
        .collect();

    let (cols, rows) = crate::render::get_terminal_size();
    let l = crate::engine::layout(cols, rows, &chars);
    crate::render::render_layout(&l);

    loop {
        match crate::input::read_event() {
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers,
                ..
            })) if modifiers.contains(KeyModifiers::CONTROL) => break,
            _ => {}
        }
    }
}
