#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CharState {
    Pending,
    Correct,
    Incorrect,
    Background,
    Border,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TypedChar {
    pub ch: char,
    pub state: CharState,
}

pub struct TextArea {
    pub row_start: u16,
    pub row_end: u16,
    pub col_start: u16,
    pub col_end: u16,
}

pub struct Layout {
    pub banner_lines: Vec<Vec<(u16, u16, char)>>,
    pub controls_lines: Vec<Vec<(u16, u16, char)>>,
    pub border_lines: Vec<(u16, u16, TypedChar)>,
    pub lines: Vec<Vec<(u16, u16, TypedChar)>>,
    pub text_area: TextArea,
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

pub const COLOR_CORRECT: &str = "\x1b[1;92m";
pub const COLOR_INCORRECT: &str = "\x1b[1;91m";
pub const COLOR_PENDING: &str = "\x1b[90m";
pub const COLOR_BG: &str = "\x1b[48;5;234m";
pub const COLOR_BORDER: &str = "\x1b[37m";
pub const COLOR_KEY: &str = "\x1b[37m";
pub const COLOR_DIM: &str = "\x1b[2m";
pub const COLOR_RESET: &str = "\x1b[0m";
