// Chess Crate
extern crate chess;
use chess::prelude::*;

// UI Crate
use notan::draw::*;
use notan::prelude::*;

// CHESS
const UNIT_COUNT: usize = 12;
// SIZES
const WIDTH: f32 = 1024.0;
const HEIGHT: f32 = 860.0;
const SQUARE_SIZE: f32 = 164.0;
// POSITIONS
const LEFT: f32 = (WIDTH - 8.0 * SQUARE_SIZE / 2.0) / 2.0;
const RIGHT: f32 = WIDTH - LEFT;
const TOP: f32 = (HEIGHT - 8.0 * SQUARE_SIZE / 2.0) / 2.0;
const BOTTOM: f32 = HEIGHT - TOP;
// COLORS
const BACKGROUND_COLOR: Color = Color::new(0.15, 0.15, 0.15, 1.0);
const WHITE_TEXT_COLOR: Color = Color::new(0.70, 0.70, 0.70, 1.0);
const BLACK_TEXT_COLOR: Color = Color::new(0.30, 0.30, 0.30, 1.0);

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
    font: Font,
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
        Unit::Pawn(Side::Black, false),
        Unit::Bishop(Side::Black),
        Unit::Knight(Side::Black),
        Unit::Rook(Side::Black, false),
        Unit::Queen(Side::Black),
        Unit::King(Side::Black, false),
        //=============
        Unit::Pawn(Side::White, false),
        Unit::Bishop(Side::White),
        Unit::Knight(Side::White),
        Unit::Rook(Side::White, false),
        Unit::Queen(Side::White),
        Unit::King(Side::White, false),
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

        println!("Unit Images loaded: {}/{}", i + 1, UNIT_COUNT);
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

    texture_buffer.push(
        gfx.create_texture()
            .from_image(include_bytes!("res/png/tile_select.png"))
            .build()
            .unwrap(),
    );

    let font = gfx
        .create_font(include_bytes!("res/font/coolvetica_condensed_rg.otf"))
        .unwrap();

    let chess = Chess::new();

    ChessState {
        chess,
        texture_buffer,
        font,
    }
}

fn update(app: &mut App, state: &mut ChessState) {
    if app.mouse.left_was_pressed() {
        match state.chess.unit_pos {
            None => state.chess.unit_pos = mouse_to_pos(app.mouse.position()),
            _ => state.chess.target_pos = mouse_to_pos(app.mouse.position()),
        }

        state.chess.background_logic();
    }
}

fn draw(gfx: &mut Graphics, state: &mut ChessState) {
    let mut draw = gfx.create_draw();
    draw.clear(BACKGROUND_COLOR);

    let mut x = LEFT;
    let mut y = TOP;
    let mut tile_idx = 0;

    for (board_y, row) in state.chess.board_state.square.into_iter().enumerate() {
        for (board_x, col) in row.into_iter().enumerate() {
            // board tile
            if tile_idx % 2 == 0 {
                draw.image(&state.texture_buffer[12]).position(x, y);
            } else {
                draw.image(&state.texture_buffer[13]).position(x, y);
            }
            // selected tile
            if let Some(pos) = state.chess.unit_pos {
                if pos == (board_x, board_y).into() {
                    draw.image(&state.texture_buffer[14]).position(x, y);
                }
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

    let text_color = match state.chess.current_turn {
        Side::Black => BLACK_TEXT_COLOR,
        Side::White => WHITE_TEXT_COLOR,
    };

    draw.text(&state.font, state.chess.current_turn.into())
        .position(WIDTH / 2.0, TOP - 50.0)
        .size(60.0)
        .color(text_color)
        .h_align_center()
        .v_align_middle();

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
