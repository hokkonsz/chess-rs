// chess crate
extern crate chess;
use chess::game::Chess;

use chess::unit::{Side, Unit};
// notan crates
use notan::draw::*;
use notan::math::{vec2, Mat3, Vec2};
use notan::prelude::*;

// positioning
const UNIT_COUNT: usize = 12;
const WIDTH: i32 = 1024;
const HEIGHT: i32 = 860;
const SQUARE_SIZE: f32 = 164.0;
// colors
const BACKGROUND_COLOR: Color = Color::new(0.416, 0.365, 0.314, 1.0);

//==================================================
//=== Application using notan
//==================================================

#[notan_main]
fn main() -> Result<(), String> {
    ChessState::run()
}

#[derive(AppState)]
pub struct ChessState {
    chess: Chess,
    texture_buffer: Vec<Texture>,
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
    let units: [Unit; UNIT_COUNT] = [
        Unit::Pawn(Side::Black),
        Unit::Bishop(Side::Black),
        Unit::Knight(Side::Black),
        Unit::Rook(Side::Black),
        Unit::Queen(Side::Black),
        Unit::King(Side::Black),
        //=============
        Unit::Pawn(Side::White),
        Unit::Bishop(Side::White),
        Unit::Knight(Side::White),
        Unit::Rook(Side::White),
        Unit::Queen(Side::White),
        Unit::King(Side::White),
    ];

    let mut texture_buffer = Vec::new();

    for (i, unit) in units.into_iter().enumerate() {
        let path = format!("res/png/{}.png", unit.get_id_str());

        texture_buffer.push(
            gfx.create_texture()
                .from_image(
                    std::fs::read(std::path::Path::new(&path))
                        .unwrap()
                        .as_slice(),
                )
                .build()
                .unwrap(),
        );

        println!("Images loaded: {}/{}", i + 1, UNIT_COUNT);
    }

    let chess = Chess::new();

    ChessState {
        chess,
        texture_buffer,
    }
}

fn draw(gfx: &mut Graphics, state: &mut ChessState) {
    let mut draw = gfx.create_draw();
    draw.clear(BACKGROUND_COLOR);

    let mut x = (WIDTH / 3) as f32;
    let mut y = (HEIGHT / 4) as f32;

    for row in state.chess.board_state.square.into_iter() {
        for col in row {
            if let Some(unit) = col {
                draw.image(&state.texture_buffer[unit.get_id() as usize])
                    .scale(0.5, 0.5)
                    .position(x, y);
            }
            x += SQUARE_SIZE;
        }
        x = (WIDTH / 3) as f32;
        y += SQUARE_SIZE;
    }

    gfx.render(&draw);
}
