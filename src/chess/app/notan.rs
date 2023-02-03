// chess crate
use crate::chess::game::Chess;
use crate::chess::unit::{Side, Unit};

// notan crates
use notan::draw::*;
use notan::math::{vec2, Mat3, Vec2};
use notan::prelude::*;

const WIDTH: i32 = 1024;
const HEIGHT: i32 = 860;

//==================================================
//=== Application using notan
//==================================================

#[derive(AppState)]
pub struct ChessState {
    chess: Chess,
    img: Texture,
}

impl ChessState {
    pub fn run() -> Result<(), String> {
        let win = WindowConfig::default()
            .size(WIDTH, HEIGHT)
            .high_dpi(true)
            .lazy_loop(true);

        notan::init_with(init)
            .add_config(win)
            .add_config(DrawConfig)
            .draw(draw)
            .build()
    }
}

fn init(gfx: &mut Graphics) -> ChessState {
    let texture = gfx
        .create_texture()
        .from_image(include_bytes!(
            "C:/Gep/Programming/Rust/_git_self/chess-rs/src/chess/png/king_w.png"
        ))
        .build()
        .unwrap();

    let chess = Chess::new();

    ChessState {
        chess,
        img: texture,
    }
}

fn draw(gfx: &mut Graphics, state: &mut ChessState) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    draw.image(&state.img)
        .scale(0.5, 0.5)
        .position((HEIGHT / 2) as f32, (WIDTH / 2) as f32);
    gfx.render(&draw);
}
