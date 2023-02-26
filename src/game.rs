// Chess Crate
use super::board::Board;
use super::pos::Pos;
use super::side::Side;

//==================================================
//=== Chess
//==================================================

/// * `board_state` current state of the board
/// * `current_turn` which side to take the next move, either [`Side::Black`] or [`Side::White`]
/// * `unit_pos` position of the unit we want to move
/// * `target_pos` target position where we want to move
pub struct Chess {
    game_state: GameState,
    pub board_state: Board,
    current_turn: Side,
    pub unit_pos: Option<Pos>,
    pub target_pos: Option<Pos>,
}

impl Chess {
    /// Creates a new Chess Game
    pub fn new() -> Self {
        Chess {
            game_state: GameState::new(),
            board_state: Board::new(),
            current_turn: Side::White,
            unit_pos: None,
            target_pos: None,
        }
    }

    /// Gives back the current game state
    pub fn get_game_state(&self) -> GameState {
        self.game_state
    }

    /// Gives back which side have to take move in the current turn
    pub fn get_current_turn(&self) -> Side {
        self.current_turn
    }

    /// Mutates the `board_state` + `current_turn` based on the value of `unit_pos` and `target_pos`
    ///
    /// Needs to be called after `unit_pos` or `target_pos` changes value.
    pub fn game_controller(&mut self) {
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
            (Some(unit_pos), Some(target_pos)) => {
                if self.board_state.step_unit(&unit_pos, &target_pos) {
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

#[derive(Clone, Copy, Debug)]
pub enum GameState {
    Playing,
    Ending(Side),
}

impl GameState {
    /// Creates a new GameState::Playing
    fn new() -> Self {
        GameState::Playing
    }
}
