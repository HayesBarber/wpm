use crate::types::{CharState, Layout, TypedChar};

const EMPTY: TypedChar = TypedChar {
    ch: ' ',
    state: CharState::Pending,
};

pub struct ScreenBuf {
    cells: Vec<Vec<TypedChar>>,
    pub rows: usize,
    pub cols: usize,
}

impl ScreenBuf {
    pub fn new(rows: usize, cols: usize) -> Self {
        ScreenBuf {
            cells: vec![vec![EMPTY; cols]; rows],
            rows,
            cols,
        }
    }

    pub fn set(&mut self, row: usize, col: usize, ch: char, state: CharState) {
        if row < self.rows && col < self.cols {
            self.cells[row][col] = TypedChar { ch, state };
        }
    }

    pub fn diff<'a>(&'a self, prev: &'a ScreenBuf) -> Vec<(u16, u16, TypedChar)> {
        let min_rows = self.rows.min(prev.rows);
        let min_cols = self.cols.min(prev.cols);
        let mut changes = Vec::new();

        for r in 0..min_rows {
            for c in 0..min_cols {
                if self.cells[r][c] != prev.cells[r][c] {
                    changes.push((r as u16, c as u16, self.cells[r][c]));
                }
            }
        }

        // Handle case where new buffer is larger than old
        for r in min_rows..self.rows {
            for c in 0..self.cols {
                changes.push((r as u16, c as u16, self.cells[r][c]));
            }
        }
        for r in 0..min_rows {
            for c in min_cols..self.cols {
                changes.push((r as u16, c as u16, self.cells[r][c]));
            }
        }

        changes
    }

    pub fn apply_layout(&mut self, layout: &Layout) {
        for line in &layout.lines {
            for &(row, col, tc) in line {
                self.set(row as usize, col as usize, tc.ch, tc.state);
            }
        }
    }
}
