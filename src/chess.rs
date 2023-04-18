// Chess Crate
use super::pos::Pos;
use super::unit::*;

const BOARD_SIZE: usize = 8;

//==================================================
//=== Board
//==================================================

#[derive(Clone)]
pub struct Board {
    pub squares: [[Option<Unit>; BOARD_SIZE]; BOARD_SIZE],
    black_king_pos: Option<Pos>,
    white_king_pos: Option<Pos>,
}

impl Board {
    /// Creates a new Default [`Board`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the [`Pos`] of the King for the given `side`
    ///
    /// If there is no King for this `side`, then returns `None`
    fn get_king_pos(&self, side: &Side) -> Option<Pos> {
        match side {
            Side::Black => self.black_king_pos,
            Side::White => self.white_king_pos,
        }
    }

    /// Sets the [`Pos`] of the King for the given `side`
    fn set_king_pos(&mut self, side: &Side, pos: Pos) {
        match side {
            Side::Black => self.black_king_pos = Some(pos),
            Side::White => self.white_king_pos = Some(pos),
        }
    }

    /// Gives back the [`Unit`] on the given position
    pub fn get_unit(&self, pos: &Pos) -> Option<Unit> {
        if !pos.is_onboard() {
            panic!("Cant get unit on Pos: {} - Pos Not On Board", pos);
        }

        self.squares[pos.y as usize][pos.x as usize]
    }

    /// Sets the [`Unit`] to the target position
    pub fn set_unit(&mut self, unit: Unit, pos: Pos) {
        if !pos.is_onboard() {
            panic!("Cant set unit on Pos: {} - Pos Not On Board", pos);
        }

        self.squares[pos.y as usize][pos.x as usize] = Some(unit);

        if let Unit::King(side, _) = unit {
            self.set_king_pos(&side, pos);
        }
    }

    /// Removes a [`Unit`] from the [`Board`]
    pub fn remove_unit(&mut self, pos: &Pos) {
        if !pos.is_onboard() {
            panic!("Cant remove unit on Pos: {} - Pos Not On Board", pos);
        }

        self.squares[pos.y as usize][pos.x as usize] = None;
    }

    /// Promotes [`Unit`] to `Queen`, when...
    /// * The type of the [`Unit`] is `Pawn`
    /// * [`Unit`] can be found in the first or the last row
    fn promote(&mut self, pos: Pos) {
        if !pos.is_onboard() {
            panic!("Cant set unit on Pos: {} - Pos Not On Board", pos);
        }

        if pos.y != 0 && pos.y != 7 {
            return;
        }

        if let Some(unit) = self.squares[pos.y as usize][pos.x as usize] {
            if eq_unit_type(&unit, &Unit::PAWN) {
                self.squares[pos.y as usize][pos.x as usize] = Some(unit.change_type(&Unit::QUEEN));
            }
        }
    }

    /// Mutates [`Board`] when called with a viable step
    pub fn test_step(&mut self, unit_pos: &Pos, target_pos: &Pos) -> bool {
        // Check Step
        let selected_unit = self.get_unit(unit_pos).unwrap();
        let step = self.step_unit(&selected_unit, unit_pos, target_pos);

        // Evaluate
        let step = step.evaluate(&self);
        if !step.is_valid() {
            return false;
        }

        // Execute Actions
        let step = step.execute_actions(self);

        // Checked Status
        if self.test_checked_status(&selected_unit.get_side()) {
            // Reset Board State
            step.reconstruct(self);
            println!("Can't move into check! / Can't move when checeked!");
            return false;
        }

        true
    }

    fn step_unit(&self, unit: &Unit, unit_pos: &Pos, target_pos: &Pos) -> Step<ConditionState> {
        match unit {
            Unit::Pawn(side, moved) => self.step_pawn(&unit_pos, &target_pos, &side, &moved),
            Unit::Bishop(side) => self.step_bishop(&unit_pos, &target_pos, &side),
            Unit::Knight(side) => self.step_knight(&unit_pos, &target_pos, &side),
            Unit::Rook(side, _) => self.step_rook(&unit_pos, &target_pos, &side),
            Unit::Queen(side) => self.step_queen(&unit_pos, &target_pos, &side),
            Unit::King(side, moved) => self.step_king(&unit_pos, &target_pos, &side, &moved),
        }
    }

    fn step_pawn(
        &self,
        unit_pos: &Pos,
        target_pos: &Pos,
        side: &Side,
        moved: &Moved,
    ) -> Step<ConditionState> {
        let mut step = Step::new(false);

        let mut calc_pos = *target_pos - *unit_pos;
        let offset_pos;

        // Validate Step Direction
        match side {
            Side::Black if calc_pos.y > 0 => offset_pos = target_pos.bounded_up(),
            Side::White if calc_pos.y < 0 => offset_pos = target_pos.bounded_down(),
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
        else if calc_pos.x == 1 && calc_pos.y == 1 {
            step.set(true);

            // Capture
            step.add_cond_pos_is_enemy(*target_pos, side);
            step.add_cond_pos_not_king(*target_pos);

            step.add_action_move(*unit_pos, *target_pos);
            step.add_action_promote(*target_pos);

            // En Passant
            step.next_group();
            step.add_cond_pos_is_enemy(offset_pos, side);
            step.add_cond_pos_not_king(offset_pos);
            step.add_cond_pos_is_none(*target_pos);

            step.add_action_remove(offset_pos);
            step.add_action_move(*unit_pos, *target_pos);
        }

        step.add_action_promote(*target_pos);

        step
    }

    fn step_bishop(&self, unit_pos: &Pos, target_pos: &Pos, side: &Side) -> Step<ConditionState> {
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

    fn step_knight(&self, unit_pos: &Pos, target_pos: &Pos, side: &Side) -> Step<ConditionState> {
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

    fn step_rook(&self, unit_pos: &Pos, target_pos: &Pos, side: &Side) -> Step<ConditionState> {
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

    fn step_queen(&self, unit_pos: &Pos, target_pos: &Pos, side: &Side) -> Step<ConditionState> {
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

    fn step_king(
        &self,
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
            step.add_cond_pos_not_king(*target_pos);

            step.add_action_move(*unit_pos, *target_pos);
        } else if calc_pos.y == 0 {
            // Castle Left E.g. E1 -> C1
            if calc_pos.x == -2 && !moved {
                step.set(true);

                for pos in unit_pos.to(&target_pos.start_row()) {
                    step.add_cond_pos_is_none(pos);
                }
                step.add_cond_pos_not_moved(target_pos.start_row());

                step.add_action_move(*unit_pos, *target_pos);
                step.add_action_move(target_pos.start_row(), target_pos.bounded_right());
            }
            // Castle Right E.g. E1 -> G1
            else if calc_pos.x == 2 && !moved {
                step.set(true);

                for pos in unit_pos.to(&target_pos.end_row()) {
                    step.add_cond_pos_is_none(pos);
                }
                step.add_cond_pos_not_moved(target_pos.end_row());

                step.add_action_move(*unit_pos, *target_pos);
                step.add_action_move(target_pos.end_row(), target_pos.bounded_left());
            }
        }

        step
    }

    /// Returns true when checked
    fn test_checked_status(&self, side: &Side) -> bool {
        let mut positions = Vec::new();

        let king_pos = self.get_king_pos(side);
        if king_pos.is_none() {
            return false;
        }
        let king_pos = king_pos.unwrap();

        // Directions
        for direction in Pos::ALL_DIRECTIONS {
            if let Some(pos) = self.find_unit_direction(king_pos, direction) {
                positions.push(pos)
            }
        }

        // Knights
        positions.extend(self.find_knights(king_pos));

        positions = positions
            .into_iter()
            .filter(|pos| self.get_unit(pos).unwrap().get_side() != *side)
            .collect();

        for pos in positions {
            let unit = self.get_unit(&pos).unwrap();

            // Check if any Unit is able to take the King
            if self.test_checking(&unit, &pos, &king_pos) {
                return true;
            }
        }

        false
    }

    /// Searches for a [`Unit`] by repeatedly calling the `step_function` on the given [`Pos`]
    ///
    /// * Returns the [`Pos`] of the [`Unit`] when found
    ///
    /// * Returns `None` otherwise
    fn find_unit_direction(&self, pos: Pos, step_function: &dyn Fn(&Pos) -> Pos) -> Option<Pos> {
        let mut check_pos = step_function(&pos);
        for _ in 0..BOARD_SIZE {
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

    /// Searches for Knight [`Unit`]s around given [`Pos`]
    ///
    /// Returns a vector, which contains all of the Knights found
    fn find_knights(&self, pos: Pos) -> Vec<Pos> {
        const KNIGHT_OFFSETS: [Pos; 8] = [
            Pos::new(-2, 1),
            Pos::new(-1, 2),
            Pos::new(1, 2),
            Pos::new(2, 1),
            Pos::new(2, -1),
            Pos::new(1, -2),
            Pos::new(-1, -2),
            Pos::new(-2, -1),
        ];

        let mut positions = Vec::new();

        for offset_pos in KNIGHT_OFFSETS {
            let check_pos = pos + offset_pos;

            if !check_pos.is_onboard() {
                continue;
            }

            if let Some(unit) = self.get_unit(&check_pos) {
                if let Unit::Knight(_) = unit {
                    positions.push(check_pos);
                }
            }
        }

        positions
    }

    /// Returns true if the [`Unit`] checks the King
    fn test_checking(&self, unit: &Unit, unit_pos: &Pos, king_pos: &Pos) -> bool {
        let calc_pos = (*king_pos - *unit_pos).abs();
        let mut valid_check = false;

        match unit {
            Unit::Pawn(side, _) => {
                // Validate Step Direction
                match side {
                    Side::Black if king_pos.y < unit_pos.y => return false,
                    Side::White if king_pos.y > unit_pos.y => return false,
                    _ => (),
                };

                if calc_pos.x == 1 && calc_pos.y == 1 {
                    valid_check = true;
                }
            }
            Unit::Bishop(_) => {
                if calc_pos.x == calc_pos.y && calc_pos.sum() != 0 {
                    valid_check = true;
                }
            }
            Unit::Knight(_) => {
                if (calc_pos.x == 1 && calc_pos.y == 2) || (calc_pos.x == 2 && calc_pos.y == 1) {
                    valid_check = true;
                }
            }
            Unit::Rook(_, _) => {
                if (calc_pos.x == 0 || calc_pos.y == 0) && calc_pos.sum() != 0 {
                    valid_check = true;
                }
            }
            Unit::Queen(_) => {
                if (calc_pos.x == 0 || calc_pos.y == 0 || calc_pos.x == calc_pos.y)
                    && calc_pos.sum() != 0
                {
                    valid_check = true;
                }
            }
            Unit::King(_, _) => {
                if calc_pos.x <= 1 && calc_pos.y <= 1 {
                    valid_check = true;
                }
            }
        }

        valid_check
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Self {
            squares: [[None; BOARD_SIZE]; BOARD_SIZE],
            black_king_pos: None,
            white_king_pos: None,
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
//================= =================================

type GroupID = i8;
type UnitPos = Pos;
type TargetPos = Pos;

trait State {}

#[derive(Debug)]
struct Step<S: State> {
    groups: GroupID,
    step_valid: bool,
    condition_state: S,
    actions: Vec<StepAction>,
}

//==================================================
//=== Step: ConditionState
//==================================================

impl State for ConditionState {}

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

    /// Adds an [`StepAction`] to [`Step`], which moves the unit from `unit_pos` to `target_pos`
    fn add_action_promote(&mut self, target_pos: Pos) {
        self.actions.push(StepAction {
            group_id: self.groups,
            command: Command::Promote(target_pos),
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
                        if !matches!(board.get_unit(&condition.pos), Some(unit) if !eq_unit_type(&Unit::KING, &unit))
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
    /// Creates a new [`ConditionState`] with empty `step_conditions`
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

impl State for ResultState {}

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
    fn execute_actions(&self, board: &mut Board) -> Step<ImageState> {
        let mut step = Step {
            groups: self.groups,
            step_valid: self.step_valid,
            condition_state: ImageState::new(),
            actions: self.actions.clone(),
        };

        // Empty GroupID
        if self.condition_state.step_result.group_id.is_none() {
            return step;
        }

        // Invalid StepResult
        if !self.condition_state.step_result.valid {
            return step;
        }

        for action in &self.actions {
            if self.condition_state.step_result.group_id.unwrap() == action.group_id {
                step.condition_state
                    .step_images
                    .extend(action.command.execute(board));
            }
        }

        step
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
    Promote(TargetPos),
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
            Self::Promote(pos) => board.promote(*pos),
        }

        step_image
    }
}

//==================================================
//=== Step: ImageState
//==================================================

impl State for ImageState {}

impl Step<ImageState> {
    /// Reconstruct the former state of a `square` in the [`Board`] from the [`StepImage`]
    fn reconstruct(&self, board: &mut Board) -> () {
        for step_image in &self.condition_state.step_images {
            if let Some(unit) = step_image.unit {
                board.set_unit(unit, step_image.pos)
            } else {
                board.remove_unit(&step_image.pos)
            }
        }
    }
}

struct ImageState {
    step_images: Vec<StepImage>,
}

impl ImageState {
    /// Creates a new [`ImageState`] with empty `step_images`
    fn new() -> Self {
        Self {
            step_images: Vec::new(),
        }
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
}

//==================================================
//=== Unit Testing
//==================================================

#[cfg(test)]
mod tests_step {
    use super::*;

    #[test]
    fn test_no_condition() {
        let step = Step::new(true);

        let step = step.evaluate(&Board::default());

        assert!(step.is_valid());
        assert_eq!(step.condition_state.step_result.group_id, Some(0));
    }

    #[test]
    fn test_invalid() {
        let mut step = Step::new(false);

        // [Group 0:] H2 -> Pawn
        step.add_cond_pos_is_none("H2".into());

        let step = step.evaluate(&Board::default());

        assert!(!step.is_valid());
        assert_eq!(step.condition_state.step_result.group_id, None);
    }

    #[test]
    fn test_empty1() {
        let mut step = Step::new(true);

        // [Group 0:] H2 -> Empty
        step.add_cond_pos_is_none("D4".into());

        // [Group 1:] H2 -> Pawn
        step.next_group();
        step.add_cond_pos_is_none("H2".into());

        // [Group 2:] D8 -> Queen
        step.next_group();
        step.add_cond_pos_is_none("D8".into());

        let step = step.evaluate(&Board::default());

        assert!(step.is_valid());
        assert_eq!(step.condition_state.step_result.group_id, Some(0));
    }

    #[test]
    fn test_empty2() {
        let mut step = Step::new(true);

        // [Group 0:] H2 -> Pawn
        step.add_cond_pos_is_none("H2".into());

        // [Group 1:] E1 -> King
        step.next_group();
        step.add_cond_pos_is_none("E1".into());

        // [Group 2:] D8 -> Queen
        step.next_group();
        step.add_cond_pos_is_none("D8".into());

        let step = step.evaluate(&Board::default());

        assert!(!step.is_valid());
        assert_eq!(step.condition_state.step_result.group_id, None);
    }

    #[test]
    fn test_empty3() {
        let mut step = Step::new(true);

        // [Group 0:] H2 -> Pawn | A2 -> Pawn
        step.add_cond_pos_is_none("H2".into());
        step.add_cond_pos_is_none("A2".into());

        // [Group 1:] D4 -> Empty | H2 -> Empty | D8 -> Empty
        step.next_group();
        step.add_cond_pos_is_none("D4".into());
        step.add_cond_pos_is_none("H4".into());
        step.add_cond_pos_is_none("D4".into());

        // [Group 2:] D4 -> Empty | H4 -> Empty | A2 -> Pawn
        step.next_group();
        step.add_cond_pos_is_none("D4".into());
        step.add_cond_pos_is_none("H4".into());
        step.add_cond_pos_is_none("A2".into());

        let step = step.evaluate(&Board::default());

        assert!(step.is_valid());
        assert_eq!(step.condition_state.step_result.group_id, Some(1));
    }

    #[test]
    fn test_empty_group1() {
        let mut step = Step::new(true);

        // Add group should do nothing here!
        step.next_group();

        // [Group 0:] D4 -> Empty | H2 -> Pawn | E4 -> Empty
        step.add_cond_pos_is_none("D4".into());
        step.add_cond_pos_is_none("H2".into());
        step.add_cond_pos_is_none("E4".into());

        // [Group 1:] D4 -> Empty | H4 -> Empty | A4 -> Empty
        step.next_group();
        step.add_cond_pos_is_none("D4".into());
        step.add_cond_pos_is_none("H4".into());
        step.add_cond_pos_is_none("A4".into());

        let step = step.evaluate(&Board::default());

        assert!(step.is_valid());
        assert_eq!(step.condition_state.step_result.group_id, Some(1));
    }

    #[test]
    fn test_empty_group2() {
        let mut step = Step::new(true);

        // [Group 0:] D4 -> Empty | H2 -> Pawn | e4 -> Empty
        step.next_group();
        step.add_cond_pos_is_none("D4".into());
        step.add_cond_pos_is_none("H2".into());
        step.add_cond_pos_is_none("E4".into());

        // [Group 1:] D4 -> Empty | E2 -> Pawn
        step.next_group();
        step.add_cond_pos_is_none("D4".into());
        step.add_cond_pos_is_none("E2".into());

        // [Group 2:] Empty Group
        step.next_group();

        let step = step.evaluate(&Board::default());

        assert!(!step.is_valid());
        assert_eq!(step.condition_state.step_result.group_id, None);
    }

    #[test]
    fn test_last_condition_valid() {
        let mut step = Step::new(true);

        // [Group 0:] B8 -> Black Knight
        step.add_cond_pos_is_enemy("B8".into(), &Side::Black);

        // [Group 1:] B1 -> White Knight
        step.next_group();
        step.add_cond_pos_is_enemy_or_none("B1".into(), &Side::White);

        // [Group 2:] F7 -> Pawn
        step.next_group();
        step.add_cond_pos_is_none("F7".into());

        // [Group 3:] E8 -> King
        step.next_group();
        step.add_cond_pos_not_king("E8".into());

        // [Group 4:] H8 -> Rook (Moved = false)
        step.next_group();
        step.add_cond_pos_not_moved("H8".into());

        let step = step.evaluate(&Board::default());

        assert!(step.is_valid());
        assert_eq!(step.condition_state.step_result.group_id, Some(4));
    }
}
