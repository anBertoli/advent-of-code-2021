use std::fmt::Debug;
use std::ops::{Add, Index};

#[derive(Debug)]
pub struct Grid<T> {
    grid: [[(T, bool); 5]; 5],
    win_row: Option<usize>,
    win_col: Option<usize>,
}

impl<T: Copy + PartialEq + Debug> Grid<T> {
    pub fn from_values(values: &[T; 25]) -> Grid<T> {
        let mut grid = [[(values[0], false); 5]; 5];
        for row in 0..5 {
            for col in 0..5 {
                let i = row * 5 + col;
                grid[row][col] = (values[i], false);
            }
        }

        Grid {
            grid,
            win_row: None,
            win_col: None,
        }
    }

    pub fn set_marked(&mut self, val: T) -> bool {
        let mut done = false;

        for row in 0..5 {
            for col in 0..5 {
                if self.grid[row][col].0 != val {
                    continue;
                }
                self.grid[row][col].1 = true;
                if self.is_row_done(row) {
                    done = true;
                    self.win_row = Some(row)
                }
                if self.is_col_done(col) {
                    done = true;
                    self.win_col = Some(col)
                }
            }
        }

        done
    }

    pub fn reset_marks(&mut self) {
        for row in 0..5 {
            for col in 0..5 {
                self.grid[row][col].1 = false;
            }
        }
    }

    pub fn is_done(&self) -> bool {
        match (self.win_row, self.win_col) {
            (None, None) => false,
            _ => true,
        }
    }

    fn is_row_done(&self, row: usize) -> bool {
        for col in 0..5 {
            if self.grid[row][col].1 == false {
                return false;
            };
        }
        true
    }

    fn is_col_done(&self, col: usize) -> bool {
        for row in 0..5 {
            if self.grid[row][col].1 == false {
                return false;
            };
        }
        true
    }
}

impl<T: Copy + PartialEq + Add<Output = T> + Default + Debug> Grid<T> {
    pub fn sum_of_unmarked(&self) -> T {
        let mut sum: T = T::default();
        for row in 0..5 {
            for col in 0..5 {
                // println!("{:?}", self.grid[row][col]);
                if !self.grid[row][col].1 {
                    sum = sum + self.grid[row][col].0;
                }
            }
        }
        sum
    }
}
