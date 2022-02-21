use colored::Colorize;
use std::fmt;

/// Newtype pattern to define an Octopus grid of 10x10 u8.
#[derive(Clone)]
pub struct Octopuses([[u8; 10]; 10]);

/// Errors returned by Octopuses methods and functions.
#[derive(Debug)]
pub enum OctoError {
    FewValues,
}

impl Octopuses {
    /// Create a new [Octopuses](Octopuses) from a slice of numbers.
    /// Returns an error if the passed numbers are less than 100.
    /// Further values are discarded.
    pub fn from_nums<N: AsRef<[u8]>>(nums: N) -> Result<Self, OctoError> {
        if nums.as_ref().len() < 100 {
            return Err(OctoError::FewValues);
        }
        let mut octo = Octopuses([[0; 10]; 10]);
        for i in 0..100 {
            octo.0[i / 10][i % 10] = nums.as_ref()[i];
        }
        Ok(octo)
    }

    /// Performs a cycle of [Octopuses](Octopuses) charging and flashing.
    /// Returns the number of flashes.
    pub fn do_cycles(&mut self, cycles: u32) -> u32 {
        let mut flashes = 0;

        for _ in 0..cycles {
            for row in 0..10 {
                for col in 0..10 {
                    self.do_cell(row, col);
                }
            }

            for row in 0..10 {
                for col in 0..10 {
                    if self.0[row][col] >= 10 {
                        self.0[row][col] = 0;
                        flashes += 1;
                    }
                }
            }
        }

        flashes
    }

    /// Perform one step at a time and stop when all the [Octopuses](Octopuses)
    /// flashes. Returns the step where the iteration was stopped.  
    pub fn until_all_flashes(&mut self) -> u64 {
        for i in 0.. {
            let flashes = self.do_cycles(1);
            if flashes == 100 {
                return i + 1;
            }
        }
        panic!("not reachable");
    }

    /// Compute the next step of a single cell and recursively
    /// compute neighbor cells if they should flash after this.
    fn do_cell(&mut self, row: usize, col: usize) {
        self.0[row][col] += 1;
        if self.0[row][col] != 10 {
            // Either already flashed
            // or not ready to flash.
            return;
        }
        let neighbors = neighbors(row, col).into_iter().flatten();
        for (rw, cl) in neighbors {
            Self::do_cell(self, rw, cl);
        }
    }
}

impl fmt::Display for Octopuses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..10 {
            let row: [u8; 10] = self.0[i];
            for n in row {
                if n == 0 {
                    write!(f, "{} ", format!("{}", n).red().bold())?;
                } else {
                    write!(f, "{} ", format!("{}", n).green().bold())?;
                }
            }
            if i != 9 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

fn neighbors(row: usize, col: usize) -> [Option<(usize, usize)>; 8] {
    let (row, col) = (row as i32, col as i32);
    [
        (row - 1, col - 1),
        (row - 1, col),
        (row - 1, col + 1),
        (row, col - 1),
        (row, col + 1),
        (row + 1, col - 1),
        (row + 1, col),
        (row + 1, col + 1),
    ]
    .map(|(r, c)| {
        if r < 0 || r >= 10 || c < 0 || c >= 10 {
            None
        } else {
            Some((r as usize, c as usize))
        }
    })
}
