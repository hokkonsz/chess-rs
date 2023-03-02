// Chess Crate
extern crate chess;
use chess::prelude::*;

// UI Crate
use notan::draw::*;
use notan::prelude::*;

// SIZES
const SQUARE_SIZE: f32 = 84.0;

const WIDTH: f32 = 1024.0;
const HEIGHT: f32 = 860.0;

const POPUP_WIDTH: f32 = 300.0;
const POPUP_HEIGHT: f32 = 200.0;

const RESTART_WIDTH: f32 = 95.0;
const RESTART_HEIGHT: f32 = 26.0;

const QUIT_WIDTH: f32 = 42.0;
const QUIT_HEIGHT: f32 = 26.0;

// POSITIONS
const LEFT: f32 = (WIDTH - 8.0 * SQUARE_SIZE) / 2.0;
const RIGHT: f32 = WIDTH - LEFT;

const TOP: f32 = (HEIGHT - 8.0 * SQUARE_SIZE) / 2.0;
const BOTTOM: f32 = HEIGHT - TOP;

const POPUP_LEFT: f32 = WIDTH / 2.0 - POPUP_WIDTH / 2.0;
const POPUP_TOP: f32 = HEIGHT / 2.0 - POPUP_HEIGHT / 2.0;

const RESTART_LEFT: f32 = WIDTH / 2.0 - RESTART_WIDTH / 2.0;
const RESTART_TOP: f32 = POPUP_TOP + 105.0 - RESTART_HEIGHT / 2.0;

const QUIT_LEFT: f32 = WIDTH / 2.0 - QUIT_WIDTH / 2.0;
const QUIT_TOP: f32 = POPUP_TOP + 145.0 - RESTART_HEIGHT / 2.0;

// COLORS
const BACKGROUND: Color = Color::new(0.15, 0.15, 0.15, 1.0);

const BOARD_BLACK: Color = Color::new(0.30, 0.30, 0.30, 1.0);
const BOARD_WHITE: Color = Color::new(0.60, 0.60, 0.60, 1.0);
const BOARD_SELECT: Color = Color::new(0.75, 0.65, 0.25, 1.0);

const TEXT: Color = Color::new(0.50, 0.50, 0.50, 1.0);

//==================================================
//=== Application: notan
//==================================================

#[notan_main]
fn main() -> Result<(), String> {
    ChessState::run()
}

#[derive(AppState)]
pub struct ChessState {
    chess: Game,
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
    let units: [Unit; Unit::UNIT_COUNT] = [
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

    for unit in units.into_iter() {
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

        println!("Image loaded: {}", unit.get_id_str());
    }

    let font = gfx
        .create_font(include_bytes!("res/font/coolvetica_condensed_rg.otf"))
        .unwrap();

    let chess = Game::new();

    ChessState {
        chess,
        texture_buffer,
        font,
    }
}

fn update(app: &mut App, state: &mut ChessState) {
    match state.chess.get_game_state() {
        GameState::Playing => {
            // Select
            if app.mouse.left_was_pressed() {
                match state.chess.unit_pos {
                    None => state.chess.unit_pos = mouse_to_pos(app.mouse.position()),
                    _ => state.chess.target_pos = mouse_to_pos(app.mouse.position()),
                }

                state.chess.game_controller();
            }

            // Deselect
            if app.mouse.right_was_pressed() {
                state.chess.unit_pos = None
            }
        }
        GameState::Ending(_) => {
            if app.mouse.left_was_pressed() {
                if check_mouse_at_restart(app.mouse.position()) {
                    app.exit()
                }

                if check_mouse_at_quit(app.mouse.position()) {
                    app.exit()
                }
            }
        }
    }
}

fn draw(gfx: &mut Graphics, state: &mut ChessState) {
    let mut draw = gfx.create_draw();
    draw.clear(BACKGROUND);

    // Chess Board
    let mut x = LEFT;
    let mut y = TOP;
    let mut tile_idx = 0;

    for (board_y, row) in state.chess.board_state.squares.into_iter().enumerate() {
        for (board_x, col) in row.into_iter().enumerate() {
            // Board Tile
            if tile_idx % 2 == 0 {
                draw.rect((x, y), (SQUARE_SIZE, SQUARE_SIZE))
                    .fill_color(BOARD_BLACK)
                    .fill();
            } else {
                draw.rect((x, y), (SQUARE_SIZE, SQUARE_SIZE))
                    .fill_color(BOARD_WHITE)
                    .fill();
            }

            // Selected Unit Tile
            if let Some(pos) = state.chess.unit_pos {
                if pos == (board_x as i8, board_y as i8).into() {
                    draw.rect((x, y), (SQUARE_SIZE, SQUARE_SIZE))
                        .fill_color(BOARD_SELECT)
                        .fill();
                }
            }
            tile_idx += 1;

            // Unit
            if let Some(unit) = col {
                draw.image(&state.texture_buffer[unit.get_id() as usize])
                    .position(x, y);
            }

            x += SQUARE_SIZE;
        }

        tile_idx += 1;
        x = LEFT;
        y += SQUARE_SIZE;
    }

    // Text: Current Turn
    let turn_color = match state.chess.get_current_turn() {
        Side::Black => BOARD_BLACK,
        Side::White => BOARD_WHITE,
    };

    draw.text(&state.font, state.chess.get_current_turn().into())
        .position(WIDTH / 2.0, TOP - 50.0)
        .size(60.0)
        .color(turn_color)
        .h_align_center()
        .v_align_middle();

    // Text: How-to-Play
    draw.text(&state.font, "How to Play")
        .position(WIDTH / 2.0, BOTTOM + 22.0)
        .size(35.0)
        .color(TEXT)
        .h_align_center()
        .v_align_middle();

    draw.text(&state.font, "LMB - Select\nRMB - Cancel Selection")
        .position(WIDTH / 2.0, BOTTOM + 67.0)
        .size(25.0)
        .color(TEXT)
        .h_align_center()
        .v_align_middle();

    if let GameState::Ending(side) = state.chess.get_game_state() {
        // Re-Play PopUp
        draw.rect((POPUP_LEFT, POPUP_TOP), (POPUP_WIDTH, POPUP_HEIGHT))
            .fill_color(BACKGROUND)
            .fill();

        let text = match side {
            Some(side) => format!("{} won!", side),
            None => String::from("Draw!"),
        };
        draw.text(&state.font, &text)
            .position(WIDTH / 2.0, POPUP_TOP + 35.0)
            .size(50.0)
            .color(TEXT)
            .h_align_center()
            .v_align_middle();

        draw.text(&state.font, "New Game")
            .position(WIDTH / 2.0, POPUP_TOP + 105.0)
            .size(35.0)
            .color(TEXT)
            .h_align_center()
            .v_align_middle();

        draw.text(&state.font, "Quit")
            .position(WIDTH / 2.0, POPUP_TOP + 145.0)
            .size(35.0)
            .color(TEXT)
            .h_align_center()
            .v_align_middle();
    }

    gfx.render(&draw);
}

/// Calculate Mouse Position to Board Pos
fn mouse_to_pos((x, y): (f32, f32)) -> Option<Pos> {
    if x > LEFT && x < RIGHT && y > TOP && y < BOTTOM {
        let x = (x - LEFT) / (SQUARE_SIZE);
        let y = (y - TOP) / (SQUARE_SIZE);

        return Some(Pos {
            x: x as i8,
            y: y as i8,
        });
    }

    None
}

/// Checks if Mouse Position is at the Restart Button
fn check_mouse_at_restart((x, y): (f32, f32)) -> bool {
    if x > RESTART_LEFT
        && x < RESTART_LEFT + RESTART_WIDTH
        && y > RESTART_TOP
        && y < RESTART_TOP + RESTART_HEIGHT
    {
        return true;
    }

    false
}

/// Checks if Mouse Position is at the Quit Button
fn check_mouse_at_quit((x, y): (f32, f32)) -> bool {
    if x > QUIT_LEFT && x < QUIT_LEFT + QUIT_WIDTH && y > QUIT_TOP && y < QUIT_TOP + QUIT_HEIGHT {
        return true;
    }

    false
}
