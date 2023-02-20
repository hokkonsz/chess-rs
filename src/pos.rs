// Standard Crate
use std::{
    fmt,
    ops::{Add, Sub},
};

// CHESS
const BOARD_SIZE: i8 = 8;

// ASCII
const UPPERCASE_A: i8 = 65;
const ZERO: i8 = 48;

//==================================================
//=== Pos
//==================================================

/// Used to position units on the board
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pos {
    pub x: i8,
    pub y: i8,
}

impl Pos {
    /// Creates a new [`Pos`] if x and y is on the board
    ///
    /// Panics when...
    /// * `x` out of bounds
    /// * `y` out of bounds
    pub fn new(x: i8, y: i8) -> Self {
        let pos = Self { x, y };

        if !pos.is_onboard() {
            panic!("Cant create Pos: {} - Pos Not On Board", pos);
        }

        pos
    }

    /// Gives back true if [`Pos`] is bounded by `BOARD_SIZE`
    pub fn is_onboard(&self) -> bool {
        if (0..BOARD_SIZE).contains(&self.x) && (0..BOARD_SIZE).contains(&self.y) {
            true
        } else {
            false
        }
    }

    /// E.g. D4 -> D5
    pub fn up(&self) -> Self {
        Self {
            x: self.x,
            y: (self.y - 1).clamp(0, BOARD_SIZE - 1),
        }
    }

    /// E.g. D4 -> C5
    pub fn up_left(&self) -> Self {
        Self {
            x: (self.x - 1).clamp(0, BOARD_SIZE - 1),
            y: (self.y - 1).clamp(0, BOARD_SIZE - 1),
        }
    }

    /// E.g. D4 -> E5
    pub fn up_right(&self) -> Self {
        Self {
            x: (self.x + 1).clamp(0, BOARD_SIZE - 1),
            y: (self.y - 1).clamp(0, BOARD_SIZE - 1),
        }
    }

    /// E.g. D4 -> D3
    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: (self.y + 1).clamp(0, BOARD_SIZE - 1),
        }
    }

    /// E.g. D4 -> C3
    pub fn down_left(&self) -> Self {
        Self {
            x: (self.x - 1).clamp(0, BOARD_SIZE - 1),
            y: (self.y + 1).clamp(0, BOARD_SIZE - 1),
        }
    }

    /// E.g. D4 -> E3
    pub fn down_right(&self) -> Self {
        Self {
            x: (self.x + 1).clamp(0, BOARD_SIZE - 1),
            y: (self.y + 1).clamp(0, BOARD_SIZE - 1),
        }
    }

    /// E.g. D4 -> C4
    pub fn left(&self) -> Self {
        Self {
            x: (self.x - 1).clamp(0, BOARD_SIZE - 1),
            y: self.y,
        }
    }

    /// E.g. D4 -> E4
    pub fn right(&self) -> Self {
        Self {
            x: (self.x + 1).clamp(0, BOARD_SIZE - 1),
            y: self.y,
        }
    }

    /// Produces `Vec<Pos>` between two [`Pos`]
    ///
    /// The positions have to be aligned one of the following ways:
    /// * Horizontal
    /// * Vertical
    /// * Diagonal
    pub fn to(&self, pos: &Pos) -> Vec<Pos> {
        let mut positions: Vec<Pos> = Vec::new();

        if !self.is_onboard() || !pos.is_onboard() {
            return positions;
        }

        let num;
        let step_pos;
        let calc_x = pos.x - self.x;
        let calc_y = pos.y - self.y;

        match (calc_x, calc_y) {
            (x, 0) if x != 0 => {
                num = x.abs();
                step_pos = Pos {
                    x: x / x.abs(),
                    y: 0,
                }
            }

            (0, y) if y != 0 => {
                num = y.abs();
                step_pos = Pos {
                    x: 0,
                    y: y / y.abs(),
                }
            }

            (x, y) if x.abs() == y.abs() && x != 0 && y != 0 => {
                num = x.abs();
                step_pos = Pos {
                    x: x / x.abs(),
                    y: y / y.abs(),
                }
            }

            _ => {
                return positions;
            }
        }

        let mut pos = *self;
        for _ in 1..num {
            pos = pos + step_pos;
            positions.push(pos);
        }

        positions
    }

    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
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
            x: (self.x + rhs.x),
            y: (self.y + rhs.y),
        }
    }
}

impl Sub for Pos {
    type Output = Pos;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: (self.x - rhs.x),
            y: (self.y - rhs.y),
        }
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<&str> for Pos {
    /// E.g. "D5" -> (4,5) -> (3,3)
    ///
    /// Panic when...
    /// * `&str` argument is invalid
    /// * [`Pos`] would be out of bounds
    fn from(s: &str) -> Self {
        if s.len() != 2 && !s.is_ascii() {
            panic!("Cant convert from &str: {} - Length != 2 or Not ASCII", s);
        }

        let col = s.to_ascii_uppercase().as_bytes()[0] as i8;
        let row = s.to_ascii_uppercase().as_bytes()[1] as i8;

        // ASCII Space -> Array Space
        let pos = Self {
            x: col - UPPERCASE_A,
            y: BOARD_SIZE - (row - ZERO),
        };

        if !pos.is_onboard() {
            panic!("Cant convert from &str: {} - Pos Not On Board", pos);
        }

        pos
    }
}

impl From<(i8, i8)> for Pos {
    fn from(tuple: (i8, i8)) -> Self {
        let pos = Self {
            x: tuple.0,
            y: tuple.1,
        };

        if !pos.is_onboard() {
            panic!("Cant convert from (i8, i8): {} - Pos Not On Board", pos);
        }

        pos
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

    //===========

    #[test]
    fn test_to1() {
        assert_eq!(Pos::from("D2").to(&Pos::from("D2")), vec![]);
    }

    #[test]
    fn test_to2() {
        assert_eq!(Pos::from("D2").to(&Pos::from("D2")), vec![]);
    }

    #[test]
    fn test_to3() {
        assert_eq!(Pos::from("D2").to(&Pos::from("D2")), vec![]);
    }

    //===========

    #[test]
    fn test_fromstr1() {
        assert_eq!(Pos::from("D2"), Pos::new(0, 0));
    }

    #[test]
    fn test_fromstr2() {
        assert_eq!(Pos::from("D2"), Pos::new(0, 0));
    }

    #[test]
    fn test_fromstr3() {
        assert_eq!(Pos::from("D2"), Pos::new(0, 0));
    }

    #[test]
    fn test_fromstr4() {
        assert_eq!(Pos::from("D2"), Pos::new(0, 0));
    }
}
