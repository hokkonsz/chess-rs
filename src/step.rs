// Chess Crate
use super::board::Board;
use super::pos::Pos;
use super::side::Side;
use super::unit::{eq_unit_type, Unit};

type GroupID = i8;
type UnitPos = Pos;
type TargetPos = Pos;

//==================================================
//=== Step
//==================================================

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
    pub fn new(step_valid: bool) -> Self {
        Self {
            groups: 0,
            step_valid,
            condition_state: ConditionState::new(),
            actions: Vec::new(),
        }
    }

    /// Sets `step_valid` to the given value
    pub fn set(&mut self, step_valid: bool) {
        self.step_valid = step_valid;
    }

    /// Increases the number of groups in [`Step`]
    pub fn next_group(&mut self) {
        if self.groups < self.condition_state.step_conditions.len() as i8 {
            self.groups += 1;
        }
    }

    /// Adds an [`Action`] to [`Step`], which removes the unit at `pos`
    pub fn add_action_remove(&mut self, pos: Pos) {
        self.actions.push(StepAction {
            group_id: self.groups,
            command: Command::Remove(pos),
        })
    }

    /// Adds an [`Action`] to [`Step`], which moves the unit from `unit_pos` to `target_pos`
    pub fn add_action_move(&mut self, unit_pos: Pos, target_pos: Pos) {
        self.actions.push(StepAction {
            group_id: self.groups,
            command: Command::Move(unit_pos, target_pos),
        })
    }

    /// Adds a [`StepCondition`] to [`Step`]
    ///
    /// Checks if the [`Pos`] is NOT occupied by any [`Unit`]
    pub fn add_cond_pos_is_none(&mut self, pos: Pos) {
        self.condition_state.step_conditions.push(StepCondition {
            group_id: self.groups,
            pos,
            test: Test::None,
        })
    }

    /// Adds a [`StepCondition`] to [`Step`]
    ///
    /// Checks if the [`Unit`] at `pos` is an enemy to `side`
    pub fn add_cond_pos_is_enemy(&mut self, pos: Pos, side: &Side) {
        self.condition_state.step_conditions.push(StepCondition {
            group_id: self.groups,
            pos,
            test: Test::Enemy(*side),
        })
    }

    /// Adds a [`StepCondition`] to [`Step`]
    ///
    /// Checks if the [`Unit`] at `pos` is EITHER an enemy to `side` OR NOT occupied by any [`Unit`]
    pub fn add_cond_pos_is_enemy_or_none(&mut self, pos: Pos, side: &Side) {
        self.condition_state.step_conditions.push(StepCondition {
            group_id: self.groups,
            pos,
            test: Test::EnemyOrNone(*side),
        })
    }

    /// Adds a [`StepCondition`] to [`Step`]
    ///
    /// Checks if the [`Unit`] at `pos` is NOT the King Type
    pub fn add_cond_pos_not_king(&mut self, pos: Pos) {
        self.condition_state.step_conditions.push(StepCondition {
            group_id: self.groups,
            pos,
            test: Test::NotKing,
        })
    }

    /// Adds a [`StepCondition`] to [`Step`]
    ///
    /// Checks if the [`Unit`] at `pos` is NOT moved yet
    pub fn add_cond_pos_not_moved(&mut self, pos: Pos) {
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
    pub fn evaluate(&self, board: &Board) -> Step<ResultState> {
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

pub struct ConditionState {
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
    pub fn is_valid(&self) -> bool {
        self.condition_state.step_result.valid
    }

    /// Executes the [`StepAction`]s on the given board based on the [`StepResult`]
    pub fn execute_actions(&self, board: &mut Board) -> () {
        // Empty GroupID
        if self.condition_state.step_result.group_id.is_none() {
            return;
        }

        // Invalid SterResult
        if !self.condition_state.step_result.valid {
            return;
        }

        for action in &self.actions {
            if self.condition_state.step_result.group_id.unwrap() == action.group_id {
                action.command.execute(board);
            }
        }
    }
}

pub struct ResultState {
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

#[derive(Clone, Copy, Debug, PartialEq)]
struct StepResult {
    group_id: Option<GroupID>,
    valid: bool,
}

//==================================================
//=== Action & Command
//==================================================

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
    fn execute(&self, board: &mut Board) {
        match self {
            Self::Remove(pos) => board.remove_unit(pos),
            Self::Move(unit_pos, target_pos) => {
                if let Some(unit) = board.get_unit(unit_pos) {
                    board.remove_unit(unit_pos);
                    board.set_unit(unit.set_moved(true), *target_pos);
                }
            }
        }
    }
}
