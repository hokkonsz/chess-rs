// Chess Crate
use super::pos::Pos;
use super::unit::{eq_unit_type, Moved, Side, Unit};

const BOARD_SIZE: usize = 8;

//==================================================
//=== Board
//==================================================

#[derive(Clone)]
pub struct Board {
    pub square: [[Option<Unit>; BOARD_SIZE]; BOARD_SIZE],
    black_king_pos: Pos,
    white_king_pos: Pos,
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

        if let Unit::King(side, _) = unit {
            match side {
                Side::Black => self.black_king_pos = pos,
                Side::White => self.white_king_pos = pos,
            }
        }
    }

    /// Removes a [`Unit`] from the [`Board`]
    pub fn remove_unit(&mut self, pos: &Pos) {
        if !pos.is_onboard() {
            panic!("Cant remove unit on Pos: {} - Pos Not On Board", pos);
        }

        self.square[pos.y as usize][pos.x as usize] = None;
    }

    /// Mutates [`Board`] when called with a viable step
    pub fn test_step(&mut self, unit_pos: &Pos, target_pos: &Pos) -> bool {
        // Check Step
        let selected_unit = self.get_unit(unit_pos).unwrap();
        let step = self.step_unit(&selected_unit, unit_pos, target_pos, false);

        // Evaluate
        let step = step.evaluate(&self);
        if !step.is_valid() {
            return false;
        }

        // Execute Actions
        let step_images = step.execute_actions(self);

        // Checked Status
        if self.test_checked_status(&selected_unit.get_side()) {
            // Reset Board State
            for step_image in step_images {
                step_image.reset(self)
            }
            return false;
        }

        true
    }

    fn step_unit(
        &self,
        unit: &Unit,
        unit_pos: &Pos,
        target_pos: &Pos,
        king_test: bool,
    ) -> Step<ConditionState> {
        match unit {
            Unit::Pawn(side, moved) => {
                self.step_pawn(&unit_pos, &target_pos, &side, &moved, &king_test)
            }
            Unit::Bishop(side) => self.step_bishop(&unit_pos, &target_pos, &side, &king_test),
            Unit::Knight(side) => self.step_knight(&unit_pos, &target_pos, &side, &king_test),
            Unit::Rook(side, _) => self.step_rook(&unit_pos, &target_pos, &side, &king_test),
            Unit::Queen(side) => self.step_queen(&unit_pos, &target_pos, &side, &king_test),
            Unit::King(side, moved) => {
                self.step_king(&unit_pos, &target_pos, &side, &moved, &king_test)
            }
        }
    }

    fn step_pawn(
        &self,
        unit_pos: &Pos,
        target_pos: &Pos,
        side: &Side,
        moved: &Moved,
        king_test: &bool,
    ) -> Step<ConditionState> {
        let mut step = Step::new(false);

        let mut calc_pos = *target_pos - *unit_pos;
        let offset_pos;

        match side {
            Side::Black if calc_pos.y > 0 => offset_pos = target_pos.checked_up(),
            Side::White if calc_pos.y < 0 => offset_pos = target_pos.checked_down(),
            _ => return step,
        };

        // 1 Vertical Step E.g. D5 -> D4
        // 2 Vertical Step E.g. D7 -> D6 OR D7 -> D5
        calc_pos = calc_pos.abs();
        if calc_pos.x == 0 {
            // 1 Step
            if calc_pos.y == 1 {
                step.set(true);

                if *king_test {
                    step.set(false);
                    return step;
                }

                step.add_cond_pos_is_none(*target_pos);

                step.add_action_move(*unit_pos, *target_pos);
            }
            // 2 Step
            else if calc_pos.y == 2 && !moved {
                step.set(true);

                if *king_test {
                    step.set(false);
                    return step;
                }

                step.add_cond_pos_is_none(offset_pos);
                step.add_cond_pos_is_none(*target_pos);

                step.add_action_move(*unit_pos, *target_pos);
            }
        }
        // Capture E.g. D5 -> C4 OR D5 -> E4 [Condition Group ID: 0]
        // En Passant E.g. D5 -> C4 OR D5 -> E4 [Condition Group ID: 1]
        else if calc_pos.x == 1 && calc_pos.x == 1 {
            step.set(true);

            if *king_test {
                println!("You are in check, can't move with this unit!");
                return step;
            }

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

    fn step_bishop(
        &self,
        unit_pos: &Pos,
        target_pos: &Pos,
        side: &Side,
        king_test: &bool,
    ) -> Step<ConditionState> {
        let mut step = Step::new(false);

        let calc_pos = (*target_pos - *unit_pos).abs();

        // Diagonal Step
        if calc_pos.x == calc_pos.y && calc_pos.sum() != 0 {
            step.set(true);

            if *king_test {
                return step;
            }

            for pos in unit_pos.to(target_pos) {
                step.add_cond_pos_is_none(pos);
            }
            step.add_cond_pos_not_king(*target_pos);
            step.add_cond_pos_is_enemy_or_none(*target_pos, side);

            step.add_action_move(*unit_pos, *target_pos);
        }

        step
    }

    fn step_knight(
        &self,
        unit_pos: &Pos,
        target_pos: &Pos,
        side: &Side,
        king_test: &bool,
    ) -> Step<ConditionState> {
        let mut step = Step::new(false);

        let calc_pos = (*target_pos - *unit_pos).abs();

        // L Step
        if (calc_pos.x == 1 && calc_pos.y == 2) || (calc_pos.x == 2 && calc_pos.y == 1) {
            step.set(true);

            if *king_test {
                println!("You are in check, can't move with this unit!");
                return step;
            }

            step.add_cond_pos_not_king(*target_pos);
            step.add_cond_pos_is_enemy_or_none(*target_pos, side);

            step.add_action_move(*unit_pos, *target_pos);
        }

        step
    }

    fn step_rook(
        &self,
        unit_pos: &Pos,
        target_pos: &Pos,
        side: &Side,
        king_test: &bool,
    ) -> Step<ConditionState> {
        let mut step = Step::new(false);

        let calc_pos = (*target_pos - *unit_pos).abs();

        // Horizontal / Vertical Step
        if (calc_pos.x == 0 || calc_pos.y == 0) && calc_pos.sum() != 0 {
            step.set(true);

            if *king_test {
                println!("You are in check, can't move with this unit!");
                return step;
            }

            for pos in unit_pos.to(target_pos) {
                step.add_cond_pos_is_none(pos);
            }
            step.add_cond_pos_not_king(*target_pos);
            step.add_cond_pos_is_enemy_or_none(*target_pos, side);

            step.add_action_move(*unit_pos, *target_pos);
        }

        step
    }

    fn step_queen(
        &self,
        unit_pos: &Pos,
        target_pos: &Pos,
        side: &Side,
        king_test: &bool,
    ) -> Step<ConditionState> {
        let mut step = Step::new(false);

        let calc_pos = (*target_pos - *unit_pos).abs();

        // Horizontal / Vertical / Diagonal Step
        if (calc_pos.x == 0 || calc_pos.y == 0 || calc_pos.x == calc_pos.y) && calc_pos.sum() != 0 {
            step.set(true);

            if *king_test {
                println!("You are in check, can't move with this unit!");
                return step;
            }

            for pos in unit_pos.to(target_pos) {
                step.add_cond_pos_is_none(pos);
            }
            step.add_cond_pos_not_king(*target_pos);
            step.add_cond_pos_is_enemy_or_none(*target_pos, side);

            step.add_action_move(*unit_pos, *target_pos);
        }

        step
    }

    fn step_king(
        &self,
        unit_pos: &Pos,
        target_pos: &Pos,
        side: &Side,
        moved: &Moved,
        king_test: &bool,
    ) -> Step<ConditionState> {
        let mut step = Step::new(false);

        let calc_pos = *target_pos - *unit_pos;

        // 1 Area Step
        if calc_pos.x.abs() <= 1 && calc_pos.y.abs() <= 1 {
            step.set(true);

            if *king_test {
                println!("Can't move into check!");
                return step;
            }

            step.add_cond_pos_is_enemy_or_none(*target_pos, side);
            step.add_cond_pos_not_king(*target_pos);

            step.add_action_move(*unit_pos, *target_pos);
        } else if calc_pos.y == 0 {
            // Castle Left E.g. E1 -> C1
            if calc_pos.x == -2 && !moved {
                step.set(true);

                if *king_test {
                    step.set(false);
                    return step;
                }

                for pos in unit_pos.to(&target_pos.start_row()) {
                    step.add_cond_pos_is_none(pos);
                }
                step.add_cond_pos_not_moved(target_pos.start_row());

                step.add_action_move(*unit_pos, *target_pos);
                step.add_action_move(target_pos.start_row(), target_pos.checked_right());
            }
            // Castle Right E.g. E1 -> G1
            else if calc_pos.x == 2 && !moved {
                step.set(true);

                if *king_test {
                    step.set(false);
                    return step;
                }

                for pos in unit_pos.to(&target_pos.end_row()) {
                    step.add_cond_pos_is_none(pos);
                }
                step.add_cond_pos_not_moved(target_pos.end_row());

                step.add_action_move(*unit_pos, *target_pos);
                step.add_action_move(target_pos.end_row(), target_pos.checked_left());
            }
        }

        step
    }

    /// Returns true when checked
    fn test_checked_status(&self, side: &Side) -> bool {
        let mut positions = Vec::new();

        let king_pos = match side {
            Side::Black => self.black_king_pos,
            Side::White => self.white_king_pos,
        };

        // Up
        if let Some(pos) = self.find_unit_direction(king_pos, &Pos::up) {
            positions.push(pos)
        }

        // Up Right
        if let Some(pos) = self.find_unit_direction(king_pos, &Pos::up_right) {
            positions.push(pos)
        }

        // Up Left
        if let Some(pos) = self.find_unit_direction(king_pos, &Pos::up_left) {
            positions.push(pos)
        }

        // Down
        if let Some(pos) = self.find_unit_direction(king_pos, &Pos::down) {
            positions.push(pos)
        }

        // Down Left
        if let Some(pos) = self.find_unit_direction(king_pos, &Pos::down_left) {
            positions.push(pos)
        }

        // Down Right
        if let Some(pos) = self.find_unit_direction(king_pos, &Pos::down_right) {
            positions.push(pos)
        }

        // Left
        if let Some(pos) = self.find_unit_direction(king_pos, &Pos::left) {
            positions.push(pos)
        }

        // Right
        if let Some(pos) = self.find_unit_direction(king_pos, &Pos::right) {
            positions.push(pos)
        }

        // TODO! Knight

        positions = positions
            .into_iter()
            .filter(|pos| self.get_unit(pos).unwrap().get_side() != *side)
            .collect();

        for pos in positions {
            let unit = self.get_unit(&pos).unwrap();
            let king_test = self.step_unit(&unit, &pos, &king_pos, true);

            // Check if any Unit is able to take the King
            if king_test.evaluate(&self).is_valid() {
                return true;
            }
        }

        false
    }

    fn find_unit_direction(&self, pos: Pos, step_function: &dyn Fn(&Pos) -> Pos) -> Option<Pos> {
        let mut check_pos = step_function(&pos);
        loop {
            // No Unit found
            if !check_pos.is_onboard() {
                return None;
            }

            // Unit found
            if self.get_unit(&check_pos).is_some() {
                break;
            }

            check_pos = step_function(&check_pos);
        }

        Some(check_pos)
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Self {
            square: [[None; BOARD_SIZE]; BOARD_SIZE],
            black_king_pos: Pos::new(0, 0),
            white_king_pos: Pos::new(0, 0),
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

//==================================================
//=== Step
//==================================================

type GroupID = i8;
type UnitPos = Pos;
type TargetPos = Pos;

pub trait State {}
impl State for ConditionState {}
impl State for ResultState {}

#[derive(Debug)]
pub struct Step<S: State> {
    groups: GroupID,
    step_valid: bool,
    condition_state: S,
    actions: Vec<StepAction>,
}

//==================================================
//=== Step: ConditionState
//==================================================

impl Step<ConditionState> {
    /// Creates a new [`Step`] with [`ConditionState`]
    fn new(step_valid: bool) -> Self {
        Self {
            groups: 0,
            step_valid,
            condition_state: ConditionState::new(),
            actions: Vec::new(),
        }
    }

    /// Sets `step_valid` to the given value
    fn set(&mut self, step_valid: bool) {
        self.step_valid = step_valid;
    }

    /// Increases the number of groups in [`Step`]
    fn next_group(&mut self) {
        if self.groups < self.condition_state.step_conditions.len() as i8 {
            self.groups += 1;
        }
    }

    /// Adds an [`StepAction`] to [`Step`], which removes the unit at `pos`
    fn add_action_remove(&mut self, pos: Pos) {
        self.actions.push(StepAction {
            group_id: self.groups,
            command: Command::Remove(pos),
        })
    }

    /// Adds an [`StepAction`] to [`Step`], which moves the unit from `unit_pos` to `target_pos`
    fn add_action_move(&mut self, unit_pos: Pos, target_pos: Pos) {
        self.actions.push(StepAction {
            group_id: self.groups,
            command: Command::Move(unit_pos, target_pos),
        })
    }

    /// Adds a [`StepCondition`] to [`Step`]
    ///
    /// Checks if the [`Pos`] is NOT occupied by any [`Unit`]
    fn add_cond_pos_is_none(&mut self, pos: Pos) {
        self.condition_state.step_conditions.push(StepCondition {
            group_id: self.groups,
            pos,
            test: Test::None,
        })
    }

    /// Adds a [`StepCondition`] to [`Step`]
    ///
    /// Checks if the [`Unit`] at `pos` is an enemy to `side`
    fn add_cond_pos_is_enemy(&mut self, pos: Pos, side: &Side) {
        self.condition_state.step_conditions.push(StepCondition {
            group_id: self.groups,
            pos,
            test: Test::Enemy(*side),
        })
    }

    /// Adds a [`StepCondition`] to [`Step`]
    ///
    /// Checks if the [`Unit`] at `pos` is EITHER an enemy to `side` OR NOT occupied by any [`Unit`]
    fn add_cond_pos_is_enemy_or_none(&mut self, pos: Pos, side: &Side) {
        self.condition_state.step_conditions.push(StepCondition {
            group_id: self.groups,
            pos,
            test: Test::EnemyOrNone(*side),
        })
    }

    /// Adds a [`StepCondition`] to [`Step`]
    ///
    /// Checks if the [`Unit`] at `pos` is NOT the King Type
    fn add_cond_pos_not_king(&mut self, pos: Pos) {
        self.condition_state.step_conditions.push(StepCondition {
            group_id: self.groups,
            pos,
            test: Test::NotKing,
        })
    }

    /// Adds a [`StepCondition`] to [`Step`]
    ///
    /// Checks if the [`Unit`] at `pos` is NOT moved yet
    fn add_cond_pos_not_moved(&mut self, pos: Pos) {
        self.condition_state.step_conditions.push(StepCondition {
            group_id: self.groups,
            pos,
            test: Test::NotMoved,
        })
    }

    /// Evaluates the [`StepCondition`]s on the given [`Board`]
    ///
    /// [`StepCondition`]s with the same `group_id` are connected with `AND`
    ///
    /// [`StepCondition`]s with the diferent `group_id` are connected with `OR`
    fn evaluate(&self, board: &Board) -> Step<ResultState> {
        let mut step = Step {
            groups: self.groups,
            step_valid: self.step_valid,
            condition_state: ResultState::new(),
            actions: self.actions.clone(),
        };

        let mut group_valid = self.step_valid;
        let mut group_id = 0;

        for condition in self.condition_state.step_conditions.as_slice() {
            // Valid Group Found
            if condition.group_id > group_id && group_valid {
                break;
            }

            // Next Group
            if condition.group_id > group_id && !group_valid {
                group_id += 1;
                group_valid = true;
            }

            // Skip Until Next Group
            if condition.group_id == group_id && !group_valid {
                continue;
            }

            match condition.test {
                Test::None => {
                    if !board.get_unit(&condition.pos).is_none() {
                        group_valid = false;
                    }
                }
                Test::Enemy(side) => {
                    if !matches!(board.get_unit(&condition.pos), Some(unit) if unit.get_side() != side)
                    {
                        group_valid = false;
                    }
                }
                Test::EnemyOrNone(side) => {
                    if !board.get_unit(&condition.pos).is_none() {
                        if !matches!(board.get_unit(&condition.pos), Some(unit) if unit.get_side() != side)
                        {
                            group_valid = false;
                        }
                    }
                }
                Test::NotKing => {
                    if !board.get_unit(&condition.pos).is_none() {
                        if !matches!(board.get_unit(&condition.pos), Some(unit) if !eq_unit_type(&Unit::King(Side::Black, true), &unit))
                        {
                            group_valid = false;
                        }
                    }
                }
                Test::NotMoved => {
                    if !matches!(board.get_unit(&condition.pos), Some(unit) if !unit.is_moved()) {
                        group_valid = false;
                    }
                }
            }
        }

        if group_valid == true {
            step.set(true, group_id);
        }

        step
    }
}

struct ConditionState {
    step_conditions: Vec<StepCondition>,
}

impl ConditionState {
    /// Creates a mew [`ConditionState`] with empty `step_conditions`
    fn new() -> Self {
        Self {
            step_conditions: Vec::new(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct StepCondition {
    group_id: GroupID,
    pos: Pos,
    test: Test,
}

#[derive(Clone, Copy, Debug)]
enum Test {
    None,
    Enemy(Side),
    EnemyOrNone(Side),
    NotKing,
    NotMoved,
}

//==================================================
//=== Step: ResultState
//==================================================

impl Step<ResultState> {
    /// Sets [`StepResult`]'s `valid` and `group_id` values
    fn set(&mut self, result_valid: bool, group_id: GroupID) {
        self.condition_state.step_result.valid = result_valid;
        self.condition_state.step_result.group_id = Some(group_id);
    }

    /// Checks if [`StepResult`] is valid
    fn is_valid(&self) -> bool {
        self.condition_state.step_result.valid
    }

    /// Executes the [`StepAction`]s on the given board based on the [`StepResult`]
    fn execute_actions(&self, board: &mut Board) -> Vec<StepImage> {
        let mut step_image = Vec::new();

        // Empty GroupID
        if self.condition_state.step_result.group_id.is_none() {
            return step_image;
        }

        // Invalid StepResult
        if !self.condition_state.step_result.valid {
            return step_image;
        }

        for action in &self.actions {
            if self.condition_state.step_result.group_id.unwrap() == action.group_id {
                step_image.extend(action.command.execute(board));
            }
        }

        step_image
    }
}

struct ResultState {
    step_result: StepResult,
}

impl ResultState {
    /// Creates a new [`ResultState`] with an invalid `step_result`
    fn new() -> Self {
        Self {
            step_result: StepResult {
                group_id: None,
                valid: false,
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct StepResult {
    group_id: Option<GroupID>,
    valid: bool,
}

#[derive(Clone, Copy, Debug)]
struct StepAction {
    group_id: GroupID,
    command: Command,
}

#[derive(Clone, Copy, Debug)]
enum Command {
    Remove(UnitPos),
    Move(UnitPos, TargetPos),
}

impl Command {
    /// Mutates [`Board`] using the [`StepAction`]s
    ///
    /// Returns a vector of [`StepImage`] which contains the former
    fn execute(&self, board: &mut Board) -> Vec<StepImage> {
        let mut step_image = Vec::new();

        match self {
            Self::Remove(pos) => {
                step_image.push(StepImage::new(board.get_unit(pos), *pos));

                board.remove_unit(pos);
            }
            Self::Move(unit_pos, target_pos) => {
                if let Some(unit) = board.get_unit(unit_pos) {
                    step_image.push(StepImage::new(Some(unit), *unit_pos));
                    step_image.push(StepImage::new(board.get_unit(target_pos), *target_pos));

                    board.remove_unit(unit_pos);
                    board.set_unit(unit.set_moved(true), *target_pos);
                }
            }
        }

        step_image
    }
}

#[derive(Clone, Copy, Debug)]
struct StepImage {
    unit: Option<Unit>,
    pos: Pos,
}

impl StepImage {
    fn new(unit: Option<Unit>, pos: Pos) -> Self {
        Self { unit, pos }
    }

    /// Reset the board [`Board`] before the [`StepAction`]s
    fn reset(&self, board: &mut Board) -> () {
        if let Some(unit) = self.unit {
            board.set_unit(unit, self.pos)
        } else {
            board.remove_unit(&self.pos)
        }
    }
}
