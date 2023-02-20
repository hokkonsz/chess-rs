// Chess Crate
use super::pos::Pos;
use super::side::Side;
use super::unit::{Moved, Unit};
use crate::step::StepResult;

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
    pub fn get_unit(&self, pos: &Pos) -> Option<Unit> {
        if !pos.is_onboard() {
            panic!("Cant get unit on Pos: {} - Pos Not On Board", pos);
        }

        self.square[pos.y as usize][pos.x as usize]
    }

    /// Sets the [`Unit`] to the target position
    pub fn set_unit(&mut self, unit: Unit, pos: Pos) {
        if !pos.is_onboard() {
            panic!("Cant set unit on Pos: {} - Pos Not On Board", pos);
        }

        self.square[pos.y as usize][pos.x as usize] = Some(unit);
    }

    /// Removes a [`Unit`] from the [`Board`]
    pub fn remove_unit(&mut self, pos: &Pos) {
        if !pos.is_onboard() {
            panic!("Cant remove unit on Pos: {} - Pos Not On Board", pos);
        }

        self.square[pos.y as usize][pos.x as usize] = None;
    }

    /// Mutates [`Board`] when called with a viable step
    pub fn step_unit(&mut self, unit_pos: &Pos, target_pos: &Pos) -> bool {
        let selected_unit = self.get_unit(unit_pos).unwrap();
        let sr = match selected_unit {
            Unit::Pawn(side, moved) => self.check_step_pawn(&unit_pos, &target_pos, &side, &moved),
            Unit::Bishop(_) => self.check_step_bishop(&unit_pos, &target_pos),
            Unit::Knight(_) => self.check_step_knight(&unit_pos, &target_pos),
            Unit::Rook(_, _) => self.check_step_rook(&unit_pos, &target_pos),
            Unit::Queen(_) => self.check_step_queen(&unit_pos, &target_pos),
            Unit::King(side, moved) => self.check_step_king(&unit_pos, &target_pos, &side, &moved),
        };

        if !sr.evaluate(&self) {
            return false;
        }

        // Step Unit
        self.remove_unit(unit_pos);
        self.set_unit(selected_unit, *target_pos);
        true
    }

    fn check_step_pawn(
        &mut self,
        unit_pos: &Pos,
        target_pos: &Pos,
        side: &Side,
        moved: &Moved,
    ) -> StepResult {
        let mut sr = StepResult::invalid();

        let mut calc_pos = *target_pos - *unit_pos;
        let offset_pos;

        match side {
            Side::Black if calc_pos.y > 0 => offset_pos = target_pos.up(),
            Side::White if calc_pos.y < 0 => offset_pos = target_pos.down(),
            _ => return sr,
        };

        // 1 Step E.g. D5 -> D4
        // 2 Step E.g. D7 -> D6 OR D7 -> D5
        calc_pos = calc_pos.abs();
        println!("1: {}", calc_pos);
        println!("2: {}", offset_pos);
        if calc_pos.x == 0 {
            // 1 Step
            if calc_pos.y == 1 {
                sr.valid = true;
            }
            // 2 Step
            else if calc_pos.y == 2 && !moved {
                sr.valid = true;

                sr.condition_empty(offset_pos);
            }
        }
        // Capture E.g. D5 -> C4 OR D5 -> E4 [Condition Group ID: 0]
        // En Passant E.g. D5 -> C4 OR D5 -> E4 [Condition Group ID: 1]
        else if calc_pos.x == 1 && calc_pos.x == 1 {
            sr.valid = true;

            // Capture
            sr.condition_any(*target_pos);

            // En Passant
            sr.add_group();
            sr.condition_any(offset_pos);
            sr.condition_empty(*target_pos);
        }

        sr
    }

    fn check_step_bishop(&mut self, unit_pos: &Pos, target_pos: &Pos) -> StepResult {
        let mut sr = StepResult::invalid();

        let calc_pos = (*target_pos - *unit_pos).abs();

        if calc_pos.x == calc_pos.y {
            sr.valid = true;

            for pos in unit_pos.to(target_pos) {
                sr.condition_empty(pos);
            }
        }

        sr
    }

    fn check_step_knight(&mut self, unit_pos: &Pos, target_pos: &Pos) -> StepResult {
        let mut sr = StepResult::invalid();

        let calc_pos = (*target_pos - *unit_pos).abs();

        if (calc_pos.x == 1 && calc_pos.y == 2) || (calc_pos.x == 2 && calc_pos.y == 1) {
            sr.valid = true;
        }

        sr
    }

    fn check_step_rook(&mut self, unit_pos: &Pos, target_pos: &Pos) -> StepResult {
        let mut sr = StepResult::invalid();

        let calc_pos = (*target_pos - *unit_pos).abs();

        if calc_pos.x == 0 || calc_pos.y == 0 {
            sr.valid = true;

            for pos in unit_pos.to(target_pos) {
                sr.condition_empty(pos);
            }
        }

        sr
    }

    fn check_step_queen(&mut self, unit_pos: &Pos, target_pos: &Pos) -> StepResult {
        let mut sr = StepResult::invalid();

        let calc_pos = (*target_pos - *unit_pos).abs();

        if calc_pos.x == 0 || calc_pos.y == 0 || calc_pos.x == calc_pos.y {
            sr.valid = true;

            for pos in unit_pos.to(target_pos) {
                sr.condition_empty(pos);
            }
        }

        sr
    }

    fn check_step_king(
        &mut self,
        unit_pos: &Pos,
        target_pos: &Pos,
        side: &Side,
        moved: &Moved,
    ) -> StepResult {
        let mut sr = StepResult::invalid();

        let calc_pos = (*target_pos - *unit_pos).abs();

        if calc_pos.x == 0 || calc_pos.y == 0 {
            sr.valid = true;
        }

        // TODO! Castle E.g. E1 -> C1 OR G1

        sr
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
