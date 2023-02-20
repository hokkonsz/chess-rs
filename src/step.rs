// Chess Crate
use super::board::Board;
use super::pos::Pos;
use super::unit::Unit;

//==================================================
//=== Step
//==================================================

#[derive(Debug)]
pub struct StepResult {
    groups: usize,
    pub valid: bool,
    conditions: Vec<Condition>,
}

impl StepResult {
    /// Creates a valid [`StepResult`] without [`Condition`]s
    pub fn valid() -> Self {
        Self {
            groups: 0,
            valid: true,
            conditions: Vec::new(),
        }
    }

    /// Creates an invalid [`StepResult`] without [`Condition`]s
    pub fn invalid() -> Self {
        Self {
            groups: 0,
            valid: false,
            conditions: Vec::new(),
        }
    }

    /// Adds [`Condition`] to [`StepResult`]
    ///
    /// Checks if the [`Pos`] is empty
    pub fn condition_empty(&mut self, pos: Pos) {
        self.conditions.push(Condition {
            group_id: self.groups,
            pos,
            test: Test::Empty,
        })
    }

    /// Adds [`Condition`] to [`StepResult`]
    ///
    /// Checks if the [`Pos`] is occupied by any [`Unit`]
    pub fn condition_any(&mut self, pos: Pos) {
        self.conditions.push(Condition {
            group_id: self.groups,
            pos,
            test: Test::Any,
        })
    }

    /// Adds [`Condition`] to [`StepResult`]
    ///
    /// Checks if the [`Pos`] is occupied by the given [`Unit`]
    pub fn condition_unit(&mut self, pos: Pos, unit: Unit) {
        self.conditions.push(Condition {
            group_id: self.groups,
            pos,
            test: Test::Is(unit),
        })
    }

    /// Increases the number of groups in [`StepResult`]
    pub fn add_group(&mut self) {
        if self.groups < self.conditions.len() {
            self.groups += 1;
        }
    }

    /// Evaluates the [`Condition`]s on the given [`Board`]
    pub fn evaluate(&self, board: &Board) -> bool {
        if self.conditions.is_empty() {
            return self.valid;
        }

        for group_id in 0..=self.groups {
            let mut group_valid = true;
            let mut num_conditions = 0;

            for condition in self.conditions.as_slice() {
                if condition.group_id == group_id {
                    num_conditions += 1;

                    match condition.test {
                        Test::Empty => {
                            if !board.get_unit(&condition.pos).is_none() {
                                group_valid = false;
                                break;
                            }
                        }
                        Test::Any => {
                            if !board.get_unit(&condition.pos).is_some() {
                                group_valid = false;
                                break;
                            }
                        }
                        Test::Is(cond_unit) => {
                            if !matches!(board.get_unit(&condition.pos), Some(unit) if unit == cond_unit)
                            {
                                group_valid = false;
                                break;
                            }
                        }
                    }
                }
            }

            if group_valid == true && num_conditions != 0 {
                return true;
            }
        }

        false
    }
}

/// Conditions with the same `group_id` are connected with `AND`
///
/// Conditions with the diferent `group_id` are connected with `OR`
#[derive(Clone, Copy, Debug)]
pub struct Condition {
    group_id: usize,
    pos: Pos,
    test: Test,
}

// TODO! Test for Enemy Unit!

/// Test cases for condition
#[derive(Clone, Copy, Debug)]
enum Test {
    Empty,
    Any,
    Is(Unit),
}

//==================================================
//=== Unit Testing
//==================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::side::Side;

    #[test]
    fn test_no_condition() {
        let sr = StepResult::valid();

        let test_board = Board::default();
        assert!(sr.evaluate(&test_board));
    }

    #[test]
    fn test_invalid() {
        let mut sr = StepResult::invalid();

        // [Group 0:] H2 -> Pawn
        sr.condition_empty("H2".into());

        let test_board = Board::default();
        assert!(!sr.evaluate(&test_board));
    }

    #[test]
    fn test_empty1() {
        let mut sr = StepResult::valid();

        // [Group 0:] H2 -> Empty
        sr.condition_empty("D4".into());

        // [Group 1:] H2 -> Pawn
        sr.add_group();
        sr.condition_empty("H2".into());

        // [Group 2:] D8 -> Queen
        sr.add_group();
        sr.condition_empty("D8".into());

        let test_board = Board::default();
        assert!(sr.evaluate(&test_board));
    }

    #[test]
    fn test_empty2() {
        let mut sr = StepResult::valid();

        // [Group 0:] H2 -> Pawn
        sr.condition_empty("H2".into());

        // [Group 1:] E1 -> King
        sr.add_group();
        sr.condition_empty("E1".into());

        // [Group 2:] D8 -> Queen
        sr.add_group();
        sr.condition_empty("D8".into());

        let test_board = Board::default();
        assert!(!sr.evaluate(&test_board));
    }

    #[test]
    fn test_empty3() {
        let mut sr = StepResult::valid();

        // [Group 0:] H2 -> Pawn | A2 -> Pawn
        sr.condition_empty("H2".into());
        sr.condition_empty("A2".into());

        // [Group 1:] D4 -> Empty | H2 -> Empty | D8 -> Empty
        sr.add_group();
        sr.condition_empty("D4".into());
        sr.condition_empty("H4".into());
        sr.condition_empty("D4".into());

        // [Group 2:] D4 -> Empty | H4 -> Empty | A2 -> Pawn
        sr.add_group();
        sr.condition_empty("D4".into());
        sr.condition_empty("H4".into());
        sr.condition_empty("A2".into());

        let test_board = Board::default();
        assert!(sr.evaluate(&test_board));
    }

    #[test]
    fn test_empty_group1() {
        let mut sr = StepResult::valid();

        // Add group should do nothing here!
        sr.add_group();

        // [Group 0:] D4 -> Empty | H2 -> Pawn | E4 -> Empty
        sr.condition_empty("D4".into());
        sr.condition_empty("H2".into());
        sr.condition_empty("E4".into());

        // [Group 1:] D4 -> Empty | H4 -> Empty | A4 -> Empty
        sr.add_group();
        sr.condition_empty("D4".into());
        sr.condition_empty("H4".into());
        sr.condition_empty("A4".into());

        let test_board = Board::default();
        assert!(sr.evaluate(&test_board));
    }

    #[test]
    fn test_empty_group2() {
        let mut sr = StepResult::valid();

        // [Group 0:] D4 -> Empty | H2 -> Pawn | e4 -> Empty
        sr.add_group();
        sr.condition_empty("D4".into());
        sr.condition_empty("H2".into());
        sr.condition_empty("E4".into());

        // [Group 1:] D4 -> Empty | E2 -> Pawn
        sr.add_group();
        sr.condition_empty("D4".into());
        sr.condition_empty("E2".into());

        // [Group 2:] Empty Group
        sr.add_group();

        let test_board = Board::default();
        assert!(!sr.evaluate(&test_board));
    }

    #[test]
    fn test_occupied1() {
        let mut sr = StepResult::valid();

        // [Group 0:] D4 -> Empty
        sr.condition_any("A4".into());

        // [Group 1:] D2 -> Queen
        sr.add_group();
        sr.condition_any("D2".into());

        // [Group 2:] E4 -> Empty
        sr.add_group();
        sr.condition_any("E4".into());

        let test_board = Board::default();
        assert!(sr.evaluate(&test_board));
    }

    #[test]
    fn test_occupied2() {
        let mut sr = StepResult::valid();

        // [Group 0:] A4 -> Empty
        sr.condition_any("A4".into());

        // [Group 1:] H4 -> Empty
        sr.add_group();
        sr.condition_any("H4".into());

        // [Group 2:] D4 -> Empty
        sr.add_group();
        sr.condition_any("D4".into());

        let test_board = Board::default();
        assert!(!sr.evaluate(&test_board));
    }

    #[test]
    fn test_occupied3() {
        let mut sr = StepResult::valid();

        // [Group 0:] H4 -> Empty | A4 -> Empty
        sr.condition_any("H4".into());
        sr.condition_any("A4".into());

        // [Group 1:] H8 -> Rook | H2 -> Pawn | H1 -> Rook
        sr.add_group();
        sr.condition_any("H8".into());
        sr.condition_any("H2".into());
        sr.condition_any("H1".into());

        // [Group 2:] D4 -> Empty | H4 -> Empty | A4 -> Empty
        sr.add_group();
        sr.condition_any("D4".into());
        sr.condition_any("H4".into());
        sr.condition_any("A4".into());

        let test_board = Board::default();
        assert!(sr.evaluate(&test_board));
    }

    #[test]
    fn test_occupiedby1() {
        let mut sr = StepResult::valid();

        // [Group 0:] A4 -> Empty
        sr.condition_unit("A4".into(), Unit::Rook(Side::Black, false));

        // [Group 1:] D2 -> Pawn
        sr.add_group();
        sr.condition_unit("D2".into(), Unit::Rook(Side::Black, false));

        // [Group 2:] H8 -> Rook
        sr.add_group();
        sr.condition_unit("H8".into(), Unit::Rook(Side::Black, false));

        let test_board = Board::default();
        assert!(sr.evaluate(&test_board));
    }

    #[test]
    fn test_occupiedby2() {
        let mut sr = StepResult::valid();

        // [Group 0:] A4 -> Empty
        sr.condition_unit("A4".into(), Unit::Pawn(Side::Black, false));

        // [Group 1:] H2 -> White Pawn
        sr.add_group();
        sr.condition_unit("H2".into(), Unit::Pawn(Side::Black, false));

        // [Group 2:] D1 -> Queen
        sr.add_group();
        sr.condition_unit("D1".into(), Unit::Pawn(Side::Black, false));

        let test_board = Board::default();
        assert!(!sr.evaluate(&test_board));
    }
}
