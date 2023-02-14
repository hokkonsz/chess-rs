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
    /// E.g. D4 -> D5
    pub fn up(&self) -> Self {
        Self {
            x: self.x,
            y: usize::clamp(self.y - 1, 0, 8),
        }
    }

    /// E.g. D4 -> C5
    pub fn up_left(&self) -> Self {
        Self {
            x: usize::clamp(self.x - 1, 0, 8),
            y: usize::clamp(self.y - 1, 0, 8),
        }
    }

    /// E.g. D4 -> E5
    pub fn up_right(&self) -> Self {
        Self {
            x: usize::clamp(self.x + 1, 0, 8),
            y: usize::clamp(self.y - 1, 0, 8),
        }
    }

    /// E.g. D4 -> D3
    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: usize::clamp(self.y + 1, 0, 8),
        }
    }

    /// E.g. D4 -> C3
    pub fn down_left(&self) -> Self {
        Self {
            x: usize::clamp(self.x - 1, 0, 8),
            y: usize::clamp(self.y + 1, 0, 8),
        }
    }

    /// E.g. D4 -> E3
    pub fn down_right(&self) -> Self {
        Self {
            x: usize::clamp(self.x + 1, 0, 8),
            y: usize::clamp(self.y + 1, 0, 8),
        }
    }

    /// E.g. D4 -> C4
    pub fn left(&self) -> Self {
        Self {
            x: usize::clamp(self.x - 1, 0, 8),
            y: self.y,
        }
    }

    /// E.g. D4 -> E4
    pub fn right(&self) -> Self {
        Self {
            x: usize::clamp(self.x + 1, 0, 8),
            y: self.y,
        }
    }

    // TODO! Methode that produces an iterator between two points!
}

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

//==================================================
//=== Unit Testing
//==================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_up() {
        assert_eq!(Pos::from("D5"), Pos::from("D4").up());
    }

    #[test]
    fn test_up_left() {
        assert_eq!(Pos::from("C5"), Pos::from("D4").up_left());
    }

    #[test]
    fn test_up_right() {
        assert_eq!(Pos::from("E5"), Pos::from("D4").up_right());
    }

    #[test]
    fn test_down() {
        assert_eq!(Pos::from("D3"), Pos::from("D4").down());
    }

    #[test]
    fn test_down_left() {
        assert_eq!(Pos::from("C3"), Pos::from("D4").down_left());
    }

    #[test]
    fn test_down_right() {
        assert_eq!(Pos::from("E3"), Pos::from("D4").down_right());
    }

    #[test]
    fn test_left() {
        assert_eq!(Pos::from("C4"), Pos::from("D4").left());
    }

    #[test]
    fn test_right() {
        assert_eq!(Pos::from("E4"), Pos::from("D4").right());
    }
}
