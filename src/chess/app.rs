use crate::chess::unit::Side;

// chess crate
use super::chess::Chess;
use super::unit::Unit;

// egui crates
#[cfg(feature = "egui")]
use eframe::egui::{Color32, Response, RichText};
#[cfg(feature = "egui")]
use eframe::{egui, App, NativeOptions};
#[cfg(feature = "egui")]
use egui_extras::image::FitTo;
#[cfg(feature = "egui")]
use egui_extras::RetainedImage;

// notan crates
#[cfg(feature = "notan")]
use notan::prelude::*;

const UNIT_COUNT: usize = 12;

//==================================================
//=== Application using eGUI
//==================================================
#[cfg(feature = "egui")]
pub struct ChessEguiApp {
    chess: Chess,
    img_buffer: Vec<RetainedImage>,
}

#[cfg(feature = "egui")]
impl ChessEguiApp {
    pub fn new() -> Self {
        Self {
            chess: Chess::new(),
            img_buffer: ChessEguiApp::load_img_buffer(),
        }
    }

    /// Call this in your [`main`] to run a native app provided by the egui crate.
    pub fn run() {
        // setup native app
        let mut options = NativeOptions::default();
        options.centered = true;

        eframe::run_native(
            "Chess",
            options,
            Box::new(|_cc| Box::new(ChessEguiApp::new())),
        )
    }

    /// Used to load the SVG images into `img_buffer` in [`ChessApp`]
    fn load_img_buffer() -> Vec<RetainedImage> {
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

        let mut img_buffer = Vec::new();

        for (i, unit) in units.into_iter().enumerate() {
            let debug_name = format!("{}", unit.get_id_str());
            let path = format!(
                "C:/Gep/Programming/Rust/test/chess/src/chess/svg/{}.svg",
                unit.get_id_str()
            );

            img_buffer.push(
                egui_extras::image::RetainedImage::from_svg_bytes_with_size(
                    debug_name,
                    std::fs::read(path).unwrap().as_slice(),
                    FitTo::Size(50, 50),
                )
                .unwrap(),
            );

            println!("Images loaded: {}/{}", i + 1, UNIT_COUNT);
        }

        img_buffer
    }
}
#[cfg(feature = "egui")]
impl App for ChessEguiApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::Area::new("Headline")
            .anchor(egui::Align2::CENTER_TOP, [0.0, 0.0])
            .show(ctx, |ui| {
                let side = format!("{} move!", &self.chess.current_turn);
                ui.label(RichText::new(side).color(Color32::WHITE).size(28.0));
            });

        // Simple Button Grid
        egui::Area::new("Board")
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 10.0])
            .show(ctx, |ui| {
                egui::Grid::new("BoardGrid")
                    .min_col_width(57.0)
                    .min_row_height(57.0)
                    .spacing((2.0, 2.0))
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
                                        (55.0, 55.0),
                                    ));
                                } else {
                                    response =
                                        ui.add(egui::Button::new("").min_size((63.0, 63.0).into()));
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

//==================================================
//=== Application using notan
//==================================================

#[cfg(feature = "notan")]
pub struct ChessNotanApp {
    chess: Chess,
}

impl ChessNotanApp {
    pub fn run() -> Result<(), String> {
        let win = WindowConfig::default()
            .size(1024, 860)
            .high_dpi(true)
            .lazy_loop(true);

        notan::init().add_config(win).build()
    }
}
