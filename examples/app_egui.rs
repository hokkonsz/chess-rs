// Chess Crate
extern crate chess;
use chess::game::Chess;
use chess::unit::{Side, Unit};

// UI Crate
use eframe::egui::{Color32, Response, RichText};
use eframe::epaint::Vec2;
use eframe::{egui, App, NativeOptions};
use egui_extras::image::FitTo;
use egui_extras::RetainedImage;

// CHESS
const UNIT_COUNT: usize = 12;
// SIZES
const WIDTH: f32 = 1024.0;
const HEIGHT: f32 = 860.0;
const SPACING: f32 = 2.0;
const SQUARE_SIZE: f32 = 50.0;

//==================================================
//=== Application using eGUI
//==================================================

fn main() {
    ChessEguiApp::run()
}

pub struct ChessEguiApp {
    chess: Chess,
    img_buffer: Vec<RetainedImage>,
}

impl ChessEguiApp {
    pub fn new() -> Self {
        Self {
            chess: Chess::new(),
            img_buffer: ChessEguiApp::init(),
        }
    }

    /// Call this in your [`main`] to run a native app provided by the egui crate.
    pub fn run() {
        // setup native app
        let mut options = NativeOptions::default();
        options.initial_window_size = Some(Vec2::new(WIDTH, HEIGHT));
        options.resizable = false;
        options.centered = true;

        eframe::run_native(
            "Chess",
            options,
            Box::new(|_cc| Box::new(ChessEguiApp::new())),
        )
    }

    /// Used to load the SVG images into `img_buffer` in [`ChessApp`]
    fn init() -> Vec<RetainedImage> {
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

        let mut img_buffer = Vec::new();

        for (i, unit) in units.into_iter().enumerate() {
            let debug_name = format!("{}", unit.get_id_str());
            let path = format!("examples/res/svg/{}.svg", unit.get_id_str());

            img_buffer.push(
                egui_extras::image::RetainedImage::from_svg_bytes_with_size(
                    debug_name,
                    std::fs::read(std::path::Path::new(&path))
                        .unwrap()
                        .as_slice(),
                    FitTo::Size(SQUARE_SIZE as u32, SQUARE_SIZE as u32),
                )
                .unwrap(),
            );

            println!("Images loaded: {}/{}", i + 1, UNIT_COUNT);
        }

        img_buffer
    }
}
impl App for ChessEguiApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::Area::new("Headline")
            .anchor(egui::Align2::CENTER_TOP, [0.0, 25.0])
            .show(ctx, |ui| {
                let side = format!("{} move!", &self.chess.current_turn);
                ui.label(RichText::new(side).color(Color32::WHITE).size(28.0));
            });

        // Simple Button Grid
        egui::Area::new("Board")
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 10.0])
            .show(ctx, |ui| {
                egui::Grid::new("BoardGrid")
                    .min_col_width(SQUARE_SIZE + 7.0)
                    .min_row_height(SQUARE_SIZE + 7.0)
                    .spacing((SPACING, SPACING))
                    .show(ui, |ui| {
                        ui.label("");
                        ui.label(RichText::new("       A").color(Color32::WHITE).size(18.0));
                        ui.label(RichText::new("       B").color(Color32::WHITE).size(18.0));
                        ui.label(RichText::new("       C").color(Color32::WHITE).size(18.0));
                        ui.label(RichText::new("       D").color(Color32::WHITE).size(18.0));
                        ui.label(RichText::new("       E").color(Color32::WHITE).size(18.0));
                        ui.label(RichText::new("       F").color(Color32::WHITE).size(18.0));
                        ui.label(RichText::new("       G").color(Color32::WHITE).size(18.0));
                        ui.label(RichText::new("       H").color(Color32::WHITE).size(18.0));
                        ui.label("");
                        ui.end_row();

                        let mut row_num = 8;
                        for (y_pos, row) in self.chess.board_state.square.into_iter().enumerate() {
                            let side = format!("   {}", row_num);
                            ui.label(RichText::new(side).color(Color32::WHITE).size(18.0));
                            row_num -= 1;

                            for (x_pos, col) in row.into_iter().enumerate() {
                                let response: Response;
                                if let Some(unit) = col {
                                    response = ui.add(egui::ImageButton::new(
                                        self.img_buffer[unit.get_id() as usize].texture_id(ctx),
                                        (SQUARE_SIZE + 5.0, SQUARE_SIZE + 5.0),
                                    ));
                                } else {
                                    response =
                                        ui.add(egui::Button::new("").min_size(
                                            (SQUARE_SIZE + 13.0, SQUARE_SIZE + 13.0).into(),
                                        ));
                                }

                                if response.clicked() {
                                    if self.chess.unit_pos.is_none() {
                                        self.chess.unit_pos = Some((x_pos, y_pos).into());
                                        println!("{:?}", self.chess.unit_pos);
                                    } else if self.chess.target_pos.is_none() {
                                        self.chess.target_pos = Some((x_pos, y_pos).into());
                                        println!("{:?}", self.chess.target_pos);
                                    }
                                }
                            }

                            ui.label("");
                            ui.end_row();
                        }
                    });
            });

        self.chess.background_logic();

        egui::CentralPanel::default().show(ctx, |_ui| {});
    }
}
