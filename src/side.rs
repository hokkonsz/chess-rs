// Standard Crate
use std::fmt;

//==================================================
//=== Unit
//==================================================

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Side {
    Black,
    White,
}

impl Side {
    /// Mutates [`Side`] to the opposite side
    pub fn swap(&mut self) {
        if *self == Self::Black {
            *self = Self::White
        } else {
            *self = Self::Black
        }
    }

    /// Gives back an opposite [`Side`]
    pub fn oppose(&self) -> Self {
        if *self == Self::Black {
            Self::White
        } else {
            Self::Black
        }
    }
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = match self {
            Side::Black => "Black",
            Side::White => "White",
        };
        write!(f, "{}", x.to_owned())
    }
}

impl From<Side> for &str {
    fn from(side: Side) -> Self {
        match side {
            Side::Black => "Black",
            Side::White => "White",
        }
    }
}
