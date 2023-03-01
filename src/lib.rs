mod chess;
mod game;
mod pos;
// mod test;
mod unit;

pub mod prelude {
    pub use crate::game::{Game, GameState};
    pub use crate::pos::Pos;
    pub use crate::unit::{Side, Unit};
}
