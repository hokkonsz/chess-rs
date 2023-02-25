//==================================================
//=== Unit Testing: Step
//==================================================

#[cfg(test)]
mod tests_step {
    use crate::board::Board;
    use crate::side::Side;
    use crate::step::*;
    use crate::unit::Unit;

    #[test]
    fn test_no_condition() {
        let sc = Step::new(true);

        let test_board = Board::default();
        assert_eq!(sc.evaluate(&test_board), StepResult::valid(None));
    }

    #[test]
    fn test_invalid() {
        let mut sc = Step::new(false);

        // [Group 0:] H2 -> Pawn
        sc.condition_none("H2".into());

        let test_board = Board::default();
        assert_eq!(sc.evaluate(&test_board), StepResult::invalid());
    }

    #[test]
    fn test_empty1() {
        let mut sc = Step::new(true);

        // [Group 0:] H2 -> Empty
        sc.condition_none("D4".into());

        // [Group 1:] H2 -> Pawn
        sc.next_group();
        sc.condition_none("H2".into());

        // [Group 2:] D8 -> Queen
        sc.next_group();
        sc.condition_none("D8".into());

        let test_board = Board::default();
        assert_eq!(sc.evaluate(&test_board), StepResult::valid(Some(0)));
    }

    #[test]
    fn test_empty2() {
        let mut sc = Step::new(true);

        // [Group 0:] H2 -> Pawn
        sc.condition_none("H2".into());

        // [Group 1:] E1 -> King
        sc.next_group();
        sc.condition_none("E1".into());

        // [Group 2:] D8 -> Queen
        sc.next_group();
        sc.condition_none("D8".into());

        let test_board = Board::default();
        assert_eq!(sc.evaluate(&test_board), StepResult::invalid());
    }

    #[test]
    fn test_empty3() {
        let mut sc = Step::new(true);

        // [Group 0:] H2 -> Pawn | A2 -> Pawn
        sc.condition_none("H2".into());
        sc.condition_none("A2".into());

        // [Group 1:] D4 -> Empty | H2 -> Empty | D8 -> Empty
        sc.next_group();
        sc.condition_none("D4".into());
        sc.condition_none("H4".into());
        sc.condition_none("D4".into());

        // [Group 2:] D4 -> Empty | H4 -> Empty | A2 -> Pawn
        sc.next_group();
        sc.condition_none("D4".into());
        sc.condition_none("H4".into());
        sc.condition_none("A2".into());

        let test_board = Board::default();
        assert_eq!(sc.evaluate(&test_board), StepResult::valid(Some(1)));
    }

    #[test]
    fn test_empty_group1() {
        let mut sc = Step::new(true);

        // Add group should do nothing here!
        sc.next_group();

        // [Group 0:] D4 -> Empty | H2 -> Pawn | E4 -> Empty
        sc.condition_none("D4".into());
        sc.condition_none("H2".into());
        sc.condition_none("E4".into());

        // [Group 1:] D4 -> Empty | H4 -> Empty | A4 -> Empty
        sc.next_group();
        sc.condition_none("D4".into());
        sc.condition_none("H4".into());
        sc.condition_none("A4".into());

        let test_board = Board::default();
        assert_eq!(sc.evaluate(&test_board), StepResult::valid(Some(1)));
    }

    #[test]
    fn test_empty_group2() {
        let mut sc = Step::new(true);

        // [Group 0:] D4 -> Empty | H2 -> Pawn | e4 -> Empty
        sc.next_group();
        sc.condition_none("D4".into());
        sc.condition_none("H2".into());
        sc.condition_none("E4".into());

        // [Group 1:] D4 -> Empty | E2 -> Pawn
        sc.next_group();
        sc.condition_none("D4".into());
        sc.condition_none("E2".into());

        // [Group 2:] Empty Group
        sc.next_group();

        let test_board = Board::default();
        assert_eq!(sc.evaluate(&test_board), StepResult::invalid());
    }

    #[test]
    fn test_occupied1() {
        let mut sc = Step::new(true);

        // [Group 0:] D4 -> Empty
        sc.condition_any("A4".into());

        // [Group 1:] D2 -> Queen
        sc.next_group();
        sc.condition_any("D2".into());

        // [Group 2:] E4 -> Empty
        sc.next_group();
        sc.condition_any("E4".into());

        let test_board = Board::default();
        assert_eq!(sc.evaluate(&test_board), StepResult::valid(Some(1)));
    }

    #[test]
    fn test_occupied2() {
        let mut sc = Step::new(true);

        // [Group 0:] A4 -> Empty
        sc.condition_any("A4".into());

        // [Group 1:] H4 -> Empty
        sc.next_group();
        sc.condition_any("H4".into());

        // [Group 2:] D4 -> Empty
        sc.next_group();
        sc.condition_any("D4".into());

        let test_board = Board::default();
        assert_eq!(sc.evaluate(&test_board), StepResult::invalid());
    }

    #[test]
    fn test_occupied3() {
        let mut sc = Step::new(true);

        // [Group 0:] H4 -> Empty | A4 -> Empty
        sc.condition_any("H4".into());
        sc.condition_any("A4".into());

        // [Group 1:] H8 -> Rook | H2 -> Pawn | H1 -> Rook
        sc.next_group();
        sc.condition_any("H8".into());
        sc.condition_any("H2".into());
        sc.condition_any("H1".into());

        // [Group 2:] D4 -> Empty | H4 -> Empty | A4 -> Empty
        sc.next_group();
        sc.condition_any("D4".into());
        sc.condition_any("H4".into());
        sc.condition_any("A4".into());

        let test_board = Board::default();
        assert_eq!(sc.evaluate(&test_board), StepResult::valid(Some(1)));
    }

    #[test]
    fn test_occupiedby1() {
        let mut sc = Step::new(true);

        // [Group 0:] A4 -> Empty
        sc.condition_is_unit("A4".into(), Unit::Rook(Side::Black, false));

        // [Group 1:] D2 -> Pawn
        sc.next_group();
        sc.condition_is_unit("D2".into(), Unit::Rook(Side::Black, false));

        // [Group 2:] H8 -> Rook
        sc.next_group();
        sc.condition_is_unit("H8".into(), Unit::Rook(Side::Black, false));

        let test_board = Board::default();
        assert_eq!(sc.evaluate(&test_board), StepResult::valid(Some(1)));
    }

    #[test]
    fn test_occupiedby2() {
        let mut sc = Step::new(true);

        // [Group 0:] A4 -> Empty
        sc.condition_is_unit("A4".into(), Unit::Pawn(Side::Black, false));

        // [Group 1:] H2 -> White Pawn
        sc.next_group();
        sc.condition_is_unit("H2".into(), Unit::Pawn(Side::Black, false));

        // [Group 2:] D1 -> Queen
        sc.next_group();
        sc.condition_is_unit("D1".into(), Unit::Pawn(Side::Black, false));

        let test_board = Board::default();
        assert_eq!(sc.evaluate(&test_board), StepResult::invalid());
    }
}

//==================================================
//=== Unit Testing: Unit
//==================================================

#[cfg(test)]
mod tests_unit {
    use crate::side::Side;
    use crate::unit::{eq_side, eq_unit_type, Unit};

    #[test]
    fn test_moved() {
        let mut unit = Unit::Pawn(Side::Black, true);
        assert_eq!(unit.is_moved(), true);

        unit.set_moved(false);
        assert_eq!(unit.is_moved(), false);

        unit = Unit::Knight(Side::White);
        assert_eq!(unit.is_moved(), false);
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

    #[test]
    fn test_side() {
        let unit1 = Unit::Pawn(Side::Black, true);
        let mut unit2 = Unit::Pawn(Side::Black, true);
        assert_eq!(eq_side(&unit1, &unit2), true);

        unit2 = Unit::King(Side::Black, false);
        assert_eq!(eq_side(&unit1, &unit2), true);

        unit2 = Unit::Bishop(Side::Black);
        assert_eq!(eq_side(&unit1, &unit2), true);

        unit2 = Unit::Pawn(Side::White, true);
        assert_eq!(eq_side(&unit1, &unit2), false);

        unit2 = Unit::King(Side::White, false);
        assert_eq!(eq_side(&unit1, &unit2), false);

        unit2 = Unit::Bishop(Side::White);
        assert_eq!(eq_side(&unit1, &unit2), false);
    }
}

//==================================================
//=== Unit Testing: Pos
//==================================================

#[cfg(test)]
mod tests_pos {
    use crate::pos::Pos;

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
