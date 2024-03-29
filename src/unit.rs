// Standard Crate
use std::fmt;

//==================================================
//=== Unit
//==================================================

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Unit {
    Pawn(Side, Moved),
    Bishop(Side),
    Knight(Side),
    Rook(Side, Moved),
    Queen(Side),
    King(Side, Moved),
}

pub type Moved = bool;

impl Unit {
    /// Number of different units (color matters)
    pub const UNIT_COUNT: usize = 12;

    // Unit Types, use these if you don't care about `Side` or `Moved`
    pub const PAWN: Unit = Unit::Pawn(Side::Black, false);
    pub const BISHOP: Unit = Unit::Bishop(Side::Black);
    pub const KNIGHT: Unit = Unit::Knight(Side::Black);
    pub const ROOK: Unit = Unit::Rook(Side::Black, false);
    pub const QUEEN: Unit = Unit::Queen(Side::Black);
    pub const KING: Unit = Unit::King(Side::Black, false);

    /// Gives back the name of the [`Unit`]
    pub fn get_name(self) -> String {
        match self {
            Self::Pawn(..) => String::from("Pawn"),
            Self::Bishop(..) => String::from("Bishop"),
            Self::Knight(..) => String::from("Knight"),
            Self::Rook(..) => String::from("Rook"),
            Self::Queen(..) => String::from("Queen"),
            Self::King(..) => String::from("King"),
        }
    }

    /// Produces an id based on the [`Unit`] variant
    ///
    /// See also [get_id_str](#method.get_id_str)
    pub fn get_id(&self) -> u8 {
        match self {
            Self::Pawn(Side::Black, _) => 0,
            Self::Bishop(Side::Black) => 1,
            Self::Knight(Side::Black) => 2,
            Self::Rook(Side::Black, _) => 3,
            Self::Queen(Side::Black) => 4,
            Self::King(Side::Black, _) => 5,
            //=============
            Self::Pawn(Side::White, _) => 6,
            Self::Bishop(Side::White) => 7,
            Self::Knight(Side::White) => 8,
            Self::Rook(Side::White, _) => 9,
            Self::Queen(Side::White) => 10,
            Self::King(Side::White, _) => 11,
        }
    }

    /// Produces an id based on the [`Unit`] variant
    ///
    /// See also [get_id](#method.get_id)
    pub fn get_id_str(&self) -> &str {
        match self {
            Self::Pawn(Side::Black, _) => "pawn_b",
            Self::Bishop(Side::Black) => "bishop_b",
            Self::Knight(Side::Black) => "knight_b",
            Self::Rook(Side::Black, _) => "rook_b",
            Self::Queen(Side::Black) => "queen_b",
            Self::King(Side::Black, _) => "king_b",
            //=============
            Self::Pawn(Side::White, _) => "pawn_w",
            Self::Bishop(Side::White) => "bishop_w",
            Self::Knight(Side::White) => "knight_w",
            Self::Rook(Side::White, _) => "rook_w",
            Self::Queen(Side::White) => "queen_w",
            Self::King(Side::White, _) => "king_w",
        }
    }

    /// Gives back the [Side] of the [`Unit`]
    pub fn get_side(&self) -> Side {
        match self {
            Self::Pawn(side, _) => *side,
            Self::Bishop(side) => *side,
            Self::Knight(side) => *side,
            Self::Rook(side, _) => *side,
            Self::Queen(side) => *side,
            Self::King(side, _) => *side,
        }
    }

    /// Checks if the [`Unit`] in the moved
    ///
    /// Works only with Pawn, Rook and King
    ///
    /// Other [`Unit`]s are always false
    pub fn is_moved(&self) -> bool {
        match self {
            Unit::Pawn(_, moved) => *moved,
            Unit::Rook(_, moved) => *moved,
            Unit::King(_, moved) => *moved,
            _ => false,
        }
    }

    /// Sets moved status on [`Unit`]
    ///
    /// Works only with Pawn, Rook and King
    ///
    /// Other [`Unit`]s are always false
    pub fn set_moved(&self, moved: bool) -> Self {
        match self {
            Unit::Pawn(side, _) => Unit::Pawn(*side, moved),
            Unit::Rook(side, _) => Unit::Rook(*side, moved),
            Unit::King(side, _) => Unit::King(*side, moved),
            unit => *unit,
        }
    }

    /// Change the current type of [`Unit`] to the type of `other_unit`
    pub fn change_type(&self, other_unit: &Unit) -> Self {
        match other_unit {
            Unit::Pawn(..) => Unit::Pawn(self.get_side(), self.is_moved()),
            Unit::Bishop(..) => Unit::Bishop(self.get_side()),
            Unit::Knight(..) => Unit::Knight(self.get_side()),
            Unit::Rook(..) => Unit::Rook(self.get_side(), self.is_moved()),
            Unit::Queen(..) => Unit::Queen(self.get_side()),
            Unit::King(..) => Unit::Pawn(self.get_side(), self.is_moved()),
        }
    }
}

/// Checks if the two [`Unit`]s are the same type
pub fn eq_unit_type(unit1: &Unit, unit2: &Unit) -> bool {
    match (unit1, unit2) {
        (Unit::Pawn(..), Unit::Pawn(..)) => true,
        (Unit::Bishop(..), Unit::Bishop(..)) => true,
        (Unit::Knight(..), Unit::Knight(..)) => true,
        (Unit::Rook(..), Unit::Rook(..)) => true,
        (Unit::Queen(..), Unit::Queen(..)) => true,
        (Unit::King(..), Unit::King(..)) => true,
        _ => false,
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unit_fmt = match self {
            Unit::Pawn(Side::Black, _) => "♟",
            Unit::Bishop(Side::Black) => "♝",
            Unit::Knight(Side::Black) => "♞",
            Unit::Rook(Side::Black, _) => "♜",
            Unit::Queen(Side::Black) => "♛",
            Unit::King(Side::Black, _) => "♚",
            Unit::Pawn(Side::White, _) => "♙",
            Unit::Bishop(Side::White) => "♗",
            Unit::Knight(Side::White) => "♘",
            Unit::Rook(Side::White, _) => "♖",
            Unit::Queen(Side::White) => "♕",
            Unit::King(Side::White, _) => "♔",
        };
        write!(f, "{}", unit_fmt.to_owned())
    }
}

//==================================================
//=== Side
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

//==================================================
//=== Unit Testing
//==================================================

#[cfg(test)]
mod tests_unit {
    use super::*;

    #[test]
    fn test_moved() {
        let unit = Unit::Pawn(Side::Black, true);
        assert!(unit.is_moved());

        let unit = unit.set_moved(false);
        assert!(!unit.is_moved());

        let unit = Unit::Knight(Side::White);
        assert!(!unit.is_moved());
    }

    #[test]
    fn test_unit_type() {
        let unit1 = Unit::Pawn(Side::Black, true);
        let mut unit2 = Unit::Pawn(Side::Black, true);
        assert_eq!(eq_unit_type(&unit1, &unit2), true);

        unit2 = Unit::Pawn(Side::Black, false);
        assert_eq!(eq_unit_type(&unit1, &unit2), true);

        unit2 = Unit::Pawn(Side::White, true);
        assert_eq!(eq_unit_type(&unit1, &unit2), true);

        unit2 = Unit::Pawn(Side::White, false);
        assert_eq!(eq_unit_type(&unit1, &unit2), true);

        unit2 = Unit::Rook(Side::Black, true);
        assert_eq!(eq_unit_type(&unit1, &unit2), false);
    }
}
