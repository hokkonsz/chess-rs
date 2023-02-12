// Standard Crate
use std::ops::{Add, Sub};

// CHESS
const BOARD_SIZE: usize = 8;

// ASCII
const LOWERCASE_A: usize = 97;
const UPPERCASE_A: usize = 65;
const DISTANCE: usize = LOWERCASE_A - UPPERCASE_A;
const ZERO: usize = 48;

//==================================================
//=== Pos
//==================================================

/// Used to position units on the board
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn up(&self) -> Self {
        Self {
            x: usize::clamp(self.x + 1, 0, 8),
            y: self.y,
        }
    }

    pub fn down(&self) -> Self {
        Self {
            x: usize::clamp(self.x - 1, 0, 8),
            y: self.y,
        }
    }

    pub fn right(&self) -> Self {
        Self {
            x: self.x,
            y: usize::clamp(self.y + 1, 0, 8),
        }
    }

    pub fn left(&self) -> Self {
        Self {
            x: self.x,
            y: usize::clamp(self.y - 1, 0, 8),
        }
    }
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
            if col > LOWERCASE_A {
                col = col - DISTANCE;
            }

            // Chess Space -> Array Space
            Self {
                x: col - UPPERCASE_A,
                y: BOARD_SIZE - (row - ZERO),
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
