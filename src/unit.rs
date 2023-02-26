// Chess Crate
use super::side::Side;

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
    /// Gives back the name of the [`Unit`]
    pub fn get_name(self) -> String {
        match self {
            Self::Pawn(_, _) => String::from("Pawn"),
            Self::Bishop(_) => String::from("Bishop"),
            Self::Knight(_) => String::from("Knight"),
            Self::Rook(_, _) => String::from("Rook"),
            Self::Queen(_) => String::from("Queen"),
            Self::King(_, _) => String::from("King"),
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
}

/// Checks if the two [`Unit`]s are the same type
pub fn eq_unit_type(unit1: &Unit, unit2: &Unit) -> bool {
    match (unit1, unit2) {
        (Unit::Pawn(_, _), Unit::Pawn(_, _)) => true,
        (Unit::Bishop(_), Unit::Bishop(_)) => true,
        (Unit::Knight(_), Unit::Knight(_)) => true,
        (Unit::Rook(_, _), Unit::Rook(_, _)) => true,
        (Unit::Queen(_), Unit::Queen(_)) => true,
        (Unit::King(_, _), Unit::King(_, _)) => true,
        _ => false,
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = match self {
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
        write!(f, "{}", x.to_owned())
    }
}
