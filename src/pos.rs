// standard crate
use std::ops::{Add, Sub};

//==================================================
//=== Pos
//==================================================

/// Used to position units on the board
#[derive(Clone, Copy, Debug)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

/// Default [`Pos`]
/// * `x` = 0
/// * `y` = 0
impl Default for Pos {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Add for Pos {
    type Output = Pos;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Pos {
    type Output = Pos;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<&str> for Pos {
    fn from(s: &str) -> Self {
        if s.len() != 2 && !s.is_ascii() {
            Self { x: 0, y: 0 }
        } else {
            // E.g. D5 -> (68,53) - (64,48) -> (4,5)
            let mut col = s.chars().nth(0).unwrap() as usize;
            let row = s.chars().nth(1).unwrap() as usize;

            // Lower ASCII Space -> Upper ASCII Space
            if col > 96 {
                col = col - 32;
            }

            // Chess Space -> Array Space
            Self {
                x: (col - 64) - 1,
                y: 8 - (row - 48),
            }
        }
    }
}

impl From<(usize, usize)> for Pos {
    fn from(tuple: (usize, usize)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}
