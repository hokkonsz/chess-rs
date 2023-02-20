// Chess Crate
use super::board::Board;
use super::pos::Pos;
use super::side::Side;

//==================================================
//=== Chess
//==================================================

/// * `board_state` current state of the board
/// * `current_turn` which side to take the next move, either [`Side::BLACK`] or [`Side::WHITE`]
/// * `unit_pos` position of the unit we want to move
/// * `target_pos` target position where we want to move
pub struct Chess {
    pub board_state: Board,
    pub current_turn: Side,
    pub unit_pos: Option<Pos>,
    pub target_pos: Option<Pos>,
}

impl Chess {
    pub fn new() -> Self {
        Chess {
            board_state: Board::default(),
            current_turn: Side::White,
            unit_pos: None,
            target_pos: None,
        }
    }

    /// Mutates the `board_state` + `current_turn` based on the value of `unit_pos` and `target_pos`
    ///
    /// Needs to be called after `unit_pos` or `target_pos` changes value.
    pub fn background_logic(&mut self) {
        match (self.unit_pos, self.target_pos) {
            (Some(unit_pos), None) => {
                if let Some(unit) = self.board_state.get_unit(&unit_pos) {
                    if unit.get_side() != self.current_turn {
                        self.unit_pos = None;
                        println!("Not your turn!");
                    }
                } else {
                    self.unit_pos = None;
                    println!("Can't move with an empty square!");
                }
            }
            (Some(unit_pos), Some(taget_pos)) => {
                if self.board_state.step_unit(&unit_pos, &taget_pos) {
                    self.current_turn.swap();
                    println!("{} moves next!", self.current_turn);
                }
                self.unit_pos = None;
                self.target_pos = None;
            }
            _ => {}
        }
    }
}
