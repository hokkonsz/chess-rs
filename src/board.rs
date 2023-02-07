// chess crate
use super::pos::Pos;
use super::unit::{Side, Unit};

const BOARD_SIZE: usize = 8;

//==================================================
//=== Board
//==================================================

pub struct Board {
    pub square: [[Option<Unit>; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    /// Gives back the [`Unit`] on the given position
    pub fn get_unit(&mut self, pos: Pos) -> Option<Unit> {
        self.square[pos.y][pos.x]
    }

    /// Sets the [`Unit`] to the target position
    pub fn set_unit(&mut self, unit: Unit, pos: Pos) {
        self.square[pos.y][pos.x] = Some(unit);
    }

    /// Removes a [`Unit`] from the [`Board`]
    pub fn remove_unit(&mut self, pos: Pos) {
        self.square[pos.y][pos.x] = None;
    }

    /// Mutates [`Board`] when called with a viable move
    pub fn move_unit(&mut self, unit_pos: Pos, target_pos: Pos) {
        match (self.get_unit(unit_pos), self.get_unit(target_pos)) {
            (Some(selected_unit), target_unit) => {
                if selected_unit.try_move(&unit_pos, &target_pos) {
                    if let Some(target_unit) = target_unit {
                        if target_unit.get_side() == selected_unit.get_side() {
                            println!("Can't take your own units!");
                            return;
                        }
                    }

                    self.set_unit(selected_unit, target_pos);
                    self.remove_unit(unit_pos);
                }
            }
            (None, _) => {
                println!("Can't move with an empty field!");
            }
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Self {
            square: [[None; BOARD_SIZE]; BOARD_SIZE],
        };

        // Black Back Row
        board.set_unit(Unit::Rook(Side::Black), "A8".into());
        board.set_unit(Unit::Knight(Side::Black), "B8".into());
        board.set_unit(Unit::Bishop(Side::Black), "C8".into());
        board.set_unit(Unit::Queen(Side::Black), "D8".into());
        board.set_unit(Unit::King(Side::Black), "E8".into());
        board.set_unit(Unit::Bishop(Side::Black), "F8".into());
        board.set_unit(Unit::Knight(Side::Black), "G8".into());
        board.set_unit(Unit::Rook(Side::Black), "H8".into());

        // Black Front Row
        board.set_unit(Unit::Pawn(Side::Black), "A7".into());
        board.set_unit(Unit::Pawn(Side::Black), "B7".into());
        board.set_unit(Unit::Pawn(Side::Black), "C7".into());
        board.set_unit(Unit::Pawn(Side::Black), "D7".into());
        board.set_unit(Unit::Pawn(Side::Black), "E7".into());
        board.set_unit(Unit::Pawn(Side::Black), "F7".into());
        board.set_unit(Unit::Pawn(Side::Black), "G7".into());
        board.set_unit(Unit::Pawn(Side::Black), "H7".into());

        // White Back Row
        board.set_unit(Unit::Rook(Side::White), "A1".into());
        board.set_unit(Unit::Knight(Side::White), "B1".into());
        board.set_unit(Unit::Bishop(Side::White), "C1".into());
        board.set_unit(Unit::Queen(Side::White), "D1".into());
        board.set_unit(Unit::King(Side::White), "E1".into());
        board.set_unit(Unit::Bishop(Side::White), "F1".into());
        board.set_unit(Unit::Knight(Side::White), "G1".into());
        board.set_unit(Unit::Rook(Side::White), "H1".into());

        // Black Front Row
        board.set_unit(Unit::Pawn(Side::White), "A2".into());
        board.set_unit(Unit::Pawn(Side::White), "B2".into());
        board.set_unit(Unit::Pawn(Side::White), "C2".into());
        board.set_unit(Unit::Pawn(Side::White), "D2".into());
        board.set_unit(Unit::Pawn(Side::White), "E2".into());
        board.set_unit(Unit::Pawn(Side::White), "F2".into());
        board.set_unit(Unit::Pawn(Side::White), "G2".into());
        board.set_unit(Unit::Pawn(Side::White), "H2".into());

        board
    }
}
