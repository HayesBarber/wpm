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

pub struct Layout {
    pub lines: Vec<Vec<(u16, u16, TypedChar)>>,
    pub cursor_row: u16,
    pub cursor_col: u16,
}

pub const PADDING: u16 = 8;
