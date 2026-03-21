use crate::types::{Layout, Style};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub ch: char,
    pub style: Style,
}

const EMPTY_CELL: Cell = Cell {
    ch: ' ',
    style: Style::Pending,
};

pub struct ScreenBuf {
    cells: Vec<Vec<Cell>>,
    pub rows: usize,
    pub cols: usize,
}

impl ScreenBuf {
    pub fn new(rows: usize, cols: usize) -> Self {
        ScreenBuf {
            cells: vec![vec![EMPTY_CELL; cols]; rows],
            rows,
            cols,
        }
    }

    pub fn set(&mut self, row: usize, col: usize, ch: char, style: Style) {
        if row < self.rows && col < self.cols {
            self.cells[row][col] = Cell { ch, style };
        }
    }

    pub fn diff<'a>(&'a self, prev: &'a ScreenBuf) -> Vec<(u16, u16, Cell)> {
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
                self.set(row as usize, col as usize, tc.ch, tc.state.into());
            }
        }
    }
}
