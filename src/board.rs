// Chess Crate
use super::pos::Pos;
use super::side::Side;
use super::unit::{Moved, Unit};
use crate::step::*;

const BOARD_SIZE: usize = 8;

//==================================================
//=== Board
//==================================================

pub struct Board {
    pub square: [[Option<Unit>; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    /// Creates a new Default [`Board`]
    pub fn new() -> Self {
        Self::default()
    }

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
        // Check Step
        let selected_unit = self.get_unit(unit_pos).unwrap();
        let step = match selected_unit {
            Unit::Pawn(side, moved) => self.check_step_pawn(&unit_pos, &target_pos, &side, &moved),
            Unit::Bishop(side) => self.check_step_bishop(&unit_pos, &target_pos, &side),
            Unit::Knight(side) => self.check_step_knight(&unit_pos, &target_pos, &side),
            Unit::Rook(side, _) => self.check_step_rook(&unit_pos, &target_pos, &side),
            Unit::Queen(side) => self.check_step_queen(&unit_pos, &target_pos, &side),
            Unit::King(side, moved) => self.check_step_king(&unit_pos, &target_pos, &side, &moved),
        };

        // Evaluate
        let step = step.evaluate(&self);
        if !step.is_valid() {
            return false;
        }

        // Step Unit
        step.execute_actions(self);
        true
    }

    fn check_step_pawn(
        &mut self,
        unit_pos: &Pos,
        target_pos: &Pos,
        side: &Side,
        moved: &Moved,
    ) -> Step<ConditionState> {
        let mut step = Step::new(false);

        let mut calc_pos = *target_pos - *unit_pos;
        let offset_pos;

        match side {
            Side::Black if calc_pos.y > 0 => offset_pos = target_pos.up(),
            Side::White if calc_pos.y < 0 => offset_pos = target_pos.down(),
            _ => return step,
        };

        // 1 Vertical Step E.g. D5 -> D4
        // 2 Vertical Step E.g. D7 -> D6 OR D7 -> D5
        calc_pos = calc_pos.abs();
        if calc_pos.x == 0 {
            // 1 Step
            if calc_pos.y == 1 {
                step.set(true);

                step.add_cond_pos_is_none(*target_pos);

                step.add_action_move(*unit_pos, *target_pos);
            }
            // 2 Step
            else if calc_pos.y == 2 && !moved {
                step.set(true);

                step.add_cond_pos_is_none(offset_pos);
                step.add_cond_pos_is_none(*target_pos);

                step.add_action_move(*unit_pos, *target_pos);
            }
        }
        // Capture E.g. D5 -> C4 OR D5 -> E4 [Condition Group ID: 0]
        // En Passant E.g. D5 -> C4 OR D5 -> E4 [Condition Group ID: 1]
        else if calc_pos.x == 1 && calc_pos.x == 1 {
            step.set(true);

            // Capture
            step.add_cond_pos_is_enemy(*target_pos, side);
            step.add_cond_pos_not_king(*target_pos);

            step.add_action_move(*unit_pos, *target_pos);

            // En Passant
            step.next_group();
            step.add_cond_pos_is_enemy(offset_pos, side);
            step.add_cond_pos_not_king(offset_pos);
            step.add_cond_pos_is_none(*target_pos);

            step.add_action_remove(offset_pos);
            step.add_action_move(*unit_pos, *target_pos);
        }

        step
    }

    fn check_step_bishop(
        &mut self,
        unit_pos: &Pos,
        target_pos: &Pos,
        side: &Side,
    ) -> Step<ConditionState> {
        let mut step = Step::new(false);

        let calc_pos = (*target_pos - *unit_pos).abs();

        // Diagonal Step
        if calc_pos.x == calc_pos.y && calc_pos.sum() != 0 {
            step.set(true);

            for pos in unit_pos.to(target_pos) {
                step.add_cond_pos_is_none(pos);
            }
            step.add_cond_pos_not_king(*target_pos);
            step.add_cond_pos_is_enemy_or_none(*target_pos, side);

            step.add_action_move(*unit_pos, *target_pos);
        }

        step
    }

    fn check_step_knight(
        &mut self,
        unit_pos: &Pos,
        target_pos: &Pos,
        side: &Side,
    ) -> Step<ConditionState> {
        let mut step = Step::new(false);

        let calc_pos = (*target_pos - *unit_pos).abs();

        // L Step
        if (calc_pos.x == 1 && calc_pos.y == 2) || (calc_pos.x == 2 && calc_pos.y == 1) {
            step.set(true);

            step.add_cond_pos_not_king(*target_pos);
            step.add_cond_pos_is_enemy_or_none(*target_pos, side);

            step.add_action_move(*unit_pos, *target_pos);
        }

        step
    }

    fn check_step_rook(
        &mut self,
        unit_pos: &Pos,
        target_pos: &Pos,
        side: &Side,
    ) -> Step<ConditionState> {
        let mut step = Step::new(false);

        let calc_pos = (*target_pos - *unit_pos).abs();

        // Horizontal / Vertical Step
        if (calc_pos.x == 0 || calc_pos.y == 0) && calc_pos.sum() != 0 {
            step.set(true);

            for pos in unit_pos.to(target_pos) {
                step.add_cond_pos_is_none(pos);
            }
            step.add_cond_pos_not_king(*target_pos);
            step.add_cond_pos_is_enemy_or_none(*target_pos, side);

            step.add_action_move(*unit_pos, *target_pos);
        }

        step
    }

    fn check_step_queen(
        &mut self,
        unit_pos: &Pos,
        target_pos: &Pos,
        side: &Side,
    ) -> Step<ConditionState> {
        let mut step = Step::new(false);

        let calc_pos = (*target_pos - *unit_pos).abs();

        // Horizontal / Vertical / Diagonal Step
        if (calc_pos.x == 0 || calc_pos.y == 0 || calc_pos.x == calc_pos.y) && calc_pos.sum() != 0 {
            step.set(true);

            for pos in unit_pos.to(target_pos) {
                step.add_cond_pos_is_none(pos);
            }
            step.add_cond_pos_not_king(*target_pos);
            step.add_cond_pos_is_enemy_or_none(*target_pos, side);

            step.add_action_move(*unit_pos, *target_pos);
        }

        step
    }

    fn check_step_king(
        &mut self,
        unit_pos: &Pos,
        target_pos: &Pos,
        side: &Side,
        moved: &Moved,
    ) -> Step<ConditionState> {
        let mut step = Step::new(false);

        let calc_pos = *target_pos - *unit_pos;

        // 1 Area Step
        if calc_pos.x.abs() <= 1 && calc_pos.y.abs() <= 1 {
            step.set(true);

            step.add_cond_pos_is_enemy_or_none(*target_pos, side);

            step.add_action_move(*unit_pos, *target_pos);
        } else if calc_pos.y == 0 {
            // Castle Left E.g. E1 -> C1
            if calc_pos.x == -2 && !moved {
                step.set(true);

                for pos in unit_pos.to(&target_pos.left().left()) {
                    step.add_cond_pos_is_none(pos);
                }
                step.add_cond_pos_not_moved(target_pos.left().left());

                step.add_action_move(*unit_pos, *target_pos);
                step.add_action_move(target_pos.left().left(), target_pos.right());
            }
            // Castle Right E.g. E1 -> G1
            else if calc_pos.x == 2 && !moved {
                step.set(true);

                for pos in unit_pos.to(&target_pos.right()) {
                    step.add_cond_pos_is_none(pos);
                }
                step.add_cond_pos_not_moved(target_pos.right());

                step.add_action_move(*unit_pos, *target_pos);
                step.add_action_move(target_pos.right(), target_pos.left());
            }
        }

        step
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
