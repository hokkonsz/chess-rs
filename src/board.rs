// Chess Crate
use super::pos::Pos;
use super::side::Side;
use super::unit::Unit;

// CHESS
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
    pub fn move_unit(&mut self, unit_pos: Pos, target_pos: Pos) -> bool {
        match (self.get_unit(unit_pos), self.get_unit(target_pos)) {
            (Some(selected_unit), target_unit) => {
                let try_move = selected_unit.try_move(&unit_pos, &target_pos);

                if try_move.valid {
                    if let Some(target_unit) = target_unit {
                        if target_unit.get_side() == selected_unit.get_side() {
                            println!("Can't take your own units!");
                            return false;
                        }
                    }

                    self.set_unit(selected_unit, target_pos);
                    self.remove_unit(unit_pos);
                }
                true
            }
            _ => {
                println!("Can't move with an empty field!");
                false
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
        board.set_unit(Unit::Rook(Side::Black, false), "A8".into());
        board.set_unit(Unit::Knight(Side::Black), "B8".into());
        board.set_unit(Unit::Bishop(Side::Black), "C8".into());
        board.set_unit(Unit::Queen(Side::Black), "D8".into());
        board.set_unit(Unit::King(Side::Black, false), "E8".into());
        board.set_unit(Unit::Bishop(Side::Black), "F8".into());
        board.set_unit(Unit::Knight(Side::Black), "G8".into());
        board.set_unit(Unit::Rook(Side::Black, false), "H8".into());

        // Black Front Row
        board.set_unit(Unit::Pawn(Side::Black, false), "A7".into());
        board.set_unit(Unit::Pawn(Side::Black, false), "B7".into());
        board.set_unit(Unit::Pawn(Side::Black, false), "C7".into());
        board.set_unit(Unit::Pawn(Side::Black, false), "D7".into());
        board.set_unit(Unit::Pawn(Side::Black, false), "E7".into());
        board.set_unit(Unit::Pawn(Side::Black, false), "F7".into());
        board.set_unit(Unit::Pawn(Side::Black, false), "G7".into());
        board.set_unit(Unit::Pawn(Side::Black, false), "H7".into());

        // White Back Row
        board.set_unit(Unit::Rook(Side::White, false), "A1".into());
        board.set_unit(Unit::Knight(Side::White), "B1".into());
        board.set_unit(Unit::Bishop(Side::White), "C1".into());
        board.set_unit(Unit::Queen(Side::White), "D1".into());
        board.set_unit(Unit::King(Side::White, false), "E1".into());
        board.set_unit(Unit::Bishop(Side::White), "F1".into());
        board.set_unit(Unit::Knight(Side::White), "G1".into());
        board.set_unit(Unit::Rook(Side::White, false), "H1".into());

        // Black Front Row
        board.set_unit(Unit::Pawn(Side::White, false), "A2".into());
        board.set_unit(Unit::Pawn(Side::White, false), "B2".into());
        board.set_unit(Unit::Pawn(Side::White, false), "C2".into());
        board.set_unit(Unit::Pawn(Side::White, false), "D2".into());
        board.set_unit(Unit::Pawn(Side::White, false), "E2".into());
        board.set_unit(Unit::Pawn(Side::White, false), "F2".into());
        board.set_unit(Unit::Pawn(Side::White, false), "G2".into());
        board.set_unit(Unit::Pawn(Side::White, false), "H2".into());

        board
    }
}
