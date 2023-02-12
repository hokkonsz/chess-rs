// Chess Crate
use super::pos::Pos;
use super::unit::Unit;

//==================================================
//=== Move Result
//==================================================

pub struct MoveResult {
    groups: usize,
    pub valid: bool,
    pub conditions: Vec<Condition>,
}

impl MoveResult {
    pub fn failed() -> Self {
        Self {
            groups: 0,
            valid: false,
            conditions: Vec::new(),
        }
    }

    /// Adds [`Condition`] to [`MoveResult`]
    ///  
    /// Checks if the [`Pos`] is occupied by any [`Unit`]
    pub fn condition_any(&mut self, pos: Pos) {
        self.conditions.push(Condition {
            group_id: self.groups,
            pos,
            is_empty: false,
            unit: None,
        })
    }

    /// Adds [`Condition`] to [`MoveResult`]
    ///
    /// Checks if the [`Pos`] is empty
    pub fn condition_empty(&mut self, pos: Pos) {
        self.conditions.push(Condition {
            group_id: self.groups,
            pos,
            is_empty: true,
            unit: None,
        })
    }

    /// Adds [`Condition`] to [`MoveResult`]
    ///
    /// Checks if the [`Pos`] is occupied by the given [`Unit`]
    pub fn condition_unit(&mut self, pos: Pos, unit: Option<Unit>) {
        self.conditions.push(Condition {
            group_id: self.groups,
            pos,
            is_empty: false,
            unit,
        })
    }

    pub fn add_group(&mut self) {
        if self.groups <= self.conditions.len() {
            self.groups += 1;
        }
    }

    pub fn evaluate() -> bool {
        todo!()
    }
}

/// Conditions with the same `group_id` are connected with `AND`
///
/// Conditions with the diferent `group_id` are connected with `OR`
pub struct Condition {
    group_id: usize,
    pos: Pos,
    is_empty: bool,
    unit: Option<Unit>,
}
