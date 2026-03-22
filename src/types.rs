#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CharState {
    Pending,
    Correct,
    Incorrect,
    Background,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TypedChar {
    pub ch: char,
    pub state: CharState,
}

pub struct Layout {
    pub banner_lines: Vec<Vec<(u16, u16, char)>>,
    pub lines: Vec<Vec<(u16, u16, TypedChar)>>,
    pub text_area: (u16, u16, u16, u16),
    pub cursor_row: u16,
    pub cursor_col: u16,
}

pub struct TestStats {
    pub wpm: f64,
    pub accuracy: f64,
    pub errors: usize,
    pub correct: usize,
    pub total: usize,
    pub elapsed_secs: f64,
}

pub const PADDING: u16 = 8;
pub const MAX_LINE_WIDTH: u16 = 50;
pub const TEXT_BG_COLOR: u8 = 236;
