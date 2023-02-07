// standard crate
use std::fmt;

// chess crate
use super::pos::Pos;

//==================================================
//=== Unit
//==================================================

#[derive(Clone, Copy, Debug)]
pub enum Unit {
    Pawn(Side),
    Bishop(Side),
    Knight(Side),
    Rook(Side),
    Queen(Side),
    King(Side),
}

impl Unit {
    /// Gives back the [Side] of the [`Unit`]
    pub fn get_side(&self) -> Side {
        match self {
            Self::Pawn(side) => *side,
            Self::Bishop(side) => *side,
            Self::Knight(side) => *side,
            Self::Rook(side) => *side,
            Self::Queen(side) => *side,
            Self::King(side) => *side,
        }
    }

    /// Gives back the name of the [`Unit`]
    pub fn get_name(self) -> String {
        match self {
            Self::Pawn(_) => String::from("Pawn"),
            Self::Bishop(_) => String::from("Bishop"),
            Self::Knight(_) => String::from("Knight"),
            Self::Rook(_) => String::from("Rook"),
            Self::Queen(_) => String::from("Queen"),
            Self::King(_) => String::from("King"),
        }
    }

    /// Produces an id based on the [`Unit`] variant
    ///
    /// See also [get_id_str](#method.get_id_str)
    pub fn get_id(&self) -> u8 {
        match self {
            Self::Pawn(Side::Black) => 0,
            Self::Bishop(Side::Black) => 1,
            Self::Knight(Side::Black) => 2,
            Self::Rook(Side::Black) => 3,
            Self::Queen(Side::Black) => 4,
            Self::King(Side::Black) => 5,
            //=============
            Self::Pawn(Side::White) => 6,
            Self::Bishop(Side::White) => 7,
            Self::Knight(Side::White) => 8,
            Self::Rook(Side::White) => 9,
            Self::Queen(Side::White) => 10,
            Self::King(Side::White) => 11,
        }
    }

    /// Produces an id based on the [`Unit`] variant
    ///
    /// See also [get_id](#method.get_id)
    pub fn get_id_str(&self) -> &str {
        match self {
            Self::Pawn(Side::Black) => "pawn_b",
            Self::Bishop(Side::Black) => "bishop_b",
            Self::Knight(Side::Black) => "knight_b",
            Self::Rook(Side::Black) => "rook_b",
            Self::Queen(Side::Black) => "queen_b",
            Self::King(Side::Black) => "king_b",
            //=============
            Self::Pawn(Side::White) => "pawn_w",
            Self::Bishop(Side::White) => "bishop_w",
            Self::Knight(Side::White) => "knight_w",
            Self::Rook(Side::White) => "rook_w",
            Self::Queen(Side::White) => "queen_w",
            Self::King(Side::White) => "king_w",
        }
    }

    /// Gives back `true`, if the move is possible
    ///
    /// See also [try_special_move](#method.try_special_move)
    pub fn try_move(self, unit_pos: &Pos, target_pos: &Pos) -> bool {
        let calc_pos = Pos {
            x: usize::max(target_pos.x, unit_pos.x) - usize::min(target_pos.x, unit_pos.x),
            y: usize::max(target_pos.y, unit_pos.y) - usize::min(target_pos.y, unit_pos.y),
        };

        match self {
            Self::Pawn(side) => match side {
                Side::Black => {
                    // Normal Move E.g. D5 -> D4
                    ((calc_pos.y == 1 && calc_pos.x == 0)
                    // First Step E.g. D7 -> D6 OR D7 -> D5
                        || ((calc_pos.y == 1 || calc_pos.y == 2)
                            && unit_pos.y == 1
                            && calc_pos.x == 0))
                        && unit_pos.y < target_pos.y
                    // Capture E.g. D5 -> C4 OR D5 -> E4 TODO!
                    // En Passant E.g. D5 -> C4 OR D5 -> E4 TODO!
                }
                Side::White => {
                    // Normal Move E.g. D5 -> D6
                    ((calc_pos.y == 1 && calc_pos.x == 0)
                    // First Step E.g. D2 -> D3 OR D2 ->4
                        || ((calc_pos.y == 1 || calc_pos.y == 2)
                            && unit_pos.y == 6
                            && calc_pos.x == 0))
                        && unit_pos.y > target_pos.y
                    // Capture E.g. D5 -> C6 OR D5 -> E6 TODO!
                    // En Passant E.g. D5 -> C6 OR D5 -> E6 TODO!
                }
            },
            Self::Bishop(_) => calc_pos.x == calc_pos.y,
            Self::Knight(_) => {
                (calc_pos.x == 1 && calc_pos.y == 2) || (calc_pos.x == 2 && calc_pos.y == 1)
            }
            Self::Rook(_) => calc_pos.x == 0 || calc_pos.y == 0,
            Self::Queen(_) => calc_pos.x == 0 || calc_pos.y == 0 || calc_pos.x == calc_pos.y,
            Self::King(_) => calc_pos.x <= 1 && calc_pos.x <= 1,
            // Castle E.g. E1 -> C1 OR G1 TODO!
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = match self {
            Unit::Pawn(Side::Black) => "♟",
            Unit::Bishop(Side::Black) => "♝",
            Unit::Knight(Side::Black) => "♞",
            Unit::Rook(Side::Black) => "♜",
            Unit::Queen(Side::Black) => "♛",
            Unit::King(Side::Black) => "♚",
            Unit::Pawn(Side::White) => "♙",
            Unit::Bishop(Side::White) => "♗",
            Unit::Knight(Side::White) => "♘",
            Unit::Rook(Side::White) => "♖",
            Unit::Queen(Side::White) => "♕",
            Unit::King(Side::White) => "♔",
        };
        write!(f, "{}", x.to_owned())
    }
}

// TODO! Tryout Phantom Data
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Side {
    Black,
    White,
}

impl Side {
    pub fn swap(&mut self) {
        if *self == Self::Black {
            *self = Self::White
        } else {
            *self = Self::Black
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
