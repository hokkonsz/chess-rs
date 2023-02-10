// chess crate
extern crate chess;
use chess::game::Chess;
use chess::pos::Pos;
use chess::unit::{Side, Unit};

// notan crates
use notan::draw::*;
use notan::prelude::*;

// init
const UNIT_COUNT: usize = 12;
// positioning
const WIDTH: f32 = 1024.0;
const HEIGHT: f32 = 860.0;
const SQUARE_SIZE: f32 = 164.0;
const LEFT: f32 = (WIDTH - 8.0 * SQUARE_SIZE / 2.0) / 2.0;
const RIGHT: f32 = WIDTH - LEFT;
const TOP: f32 = (HEIGHT - 8.0 * SQUARE_SIZE / 2.0) / 2.0;
const BOTTOM: f32 = HEIGHT - TOP;
// colors
const BACKGROUND_COLOR: Color = Color::new(0.93, 0.90, 0.87, 1.0);

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
            .size(WIDTH as i32, HEIGHT as i32)
            .high_dpi(true)
            .lazy_loop(true);

        notan::init_with(init)
            .add_config(win)
            .add_config(DrawConfig)
            .update(update)
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
        let path = format!("examples/res/png/{}.png", unit.get_id_str());

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

    texture_buffer.push(
        gfx.create_texture()
            .from_image(include_bytes!("res/png/tile_b.png"))
            .build()
            .unwrap(),
    );

    texture_buffer.push(
        gfx.create_texture()
            .from_image(include_bytes!("res/png/tile_w.png"))
            .build()
            .unwrap(),
    );

    let chess = Chess::new();

    ChessState {
        chess,
        texture_buffer,
    }
}

fn update(app: &mut App, state: &mut ChessState) {
    if app.mouse.left_was_pressed() {
        state.chess.unit_pos = mouse_to_pos(app.mouse.position());
    } else if app.mouse.left_was_released() {
        state.chess.target_pos = mouse_to_pos(app.mouse.position());
    }

    state.chess.background_logic();
}

fn draw(gfx: &mut Graphics, state: &mut ChessState) {
    let mut draw = gfx.create_draw();
    draw.clear(BACKGROUND_COLOR);

    let mut x = LEFT;
    let mut y = TOP;
    let mut tile_idx = 0;

    for row in state.chess.board_state.square.into_iter() {
        for col in row {
            // tile
            if tile_idx % 2 == 0 {
                draw.image(&state.texture_buffer[12]).position(x, y);
            } else {
                draw.image(&state.texture_buffer[13]).position(x, y);
            }
            tile_idx += 1;

            // unit
            if let Some(unit) = col {
                draw.image(&state.texture_buffer[unit.get_id() as usize])
                    .position(x, y);
            }

            x += SQUARE_SIZE / 2.0;
        }

        tile_idx += 1;
        x = LEFT;
        y += SQUARE_SIZE / 2.0;
    }

    gfx.render(&draw);
}

fn mouse_to_pos((x, y): (f32, f32)) -> Option<Pos> {
    if x > LEFT && x < RIGHT && y > TOP && y < BOTTOM {
        let x = (x - LEFT) / (SQUARE_SIZE / 2.0);
        let y = (y - TOP) / (SQUARE_SIZE / 2.0);

        return Some(Pos {
            x: x as usize,
            y: y as usize,
        });
    }
    None
}
