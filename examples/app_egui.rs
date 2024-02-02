// Standard
use std::{fs, path::Path};

// Chess Crate
extern crate chess;
use chess::prelude::*;

// UI Crate
use eframe::egui::{Color32, Image, Response, RichText};
use eframe::{egui, App};

// SIZES
const WIDTH: f32 = 1024.0;
const HEIGHT: f32 = 860.0;
const SPACING: f32 = 2.0;
const SQUARE_SIZE: f32 = 50.0;

//==================================================
//=== Application: eGUI
//==================================================

fn main() {
    ChessEguiApp::run()
}

#[derive(Default)]
pub struct ChessEguiApp {
    chess: Game,
}

impl ChessEguiApp {
    pub fn new() -> Self {
        Self { chess: Game::new() }
    }

    pub fn run() {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([WIDTH, HEIGHT])
                .with_title("Chess"),
            centered: true,
            ..Default::default()
        };

        let _ = eframe::run_native(
            "Chess",
            options,
            Box::new(|cc| {
                egui_extras::install_image_loaders(&cc.egui_ctx);
                Box::<ChessEguiApp>::default()
            }),
        );
    }
}
impl App for ChessEguiApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::Area::new("Headline")
            .anchor(egui::Align2::CENTER_TOP, [0.0, 25.0])
            .show(ctx, |ui| {
                let side = format!("{} move!", &self.chess.get_current_turn());
                ui.label(RichText::new(side).color(Color32::WHITE).size(28.0));
            });

        // Simple Button Grid
        egui::Area::new("Board")
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 10.0])
            .show(ctx, |ui| {
                egui::Grid::new("BoardGrid")
                    .min_col_width(SQUARE_SIZE + 13.0)
                    .min_row_height(SQUARE_SIZE + 13.0)
                    .spacing((SPACING, SPACING))
                    .show(ui, |ui| {
                        // Top Legend
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
                        for (y_pos, row) in self.chess.board_state.squares.into_iter().enumerate() {
                            // Left Legend
                            let side = format!("   {}", row_num);
                            ui.label(RichText::new(side).color(Color32::WHITE).size(18.0));
                            row_num -= 1;

                            // Board
                            for (x_pos, col) in row.into_iter().enumerate() {
                                let response: Response;
                                if let Some(unit) = col {
                                    let uri = format!(
                                        "bytes://examples/res/svg/{}.svg",
                                        unit.get_id_str()
                                    );

                                    let bytes = fs::read(Path::new(&format!(
                                        "examples/res/svg/{}.svg",
                                        unit.get_id_str()
                                    )))
                                    .unwrap();

                                    response = ui.add(egui::ImageButton::new(
                                        egui::Image::from_bytes(uri, bytes),
                                    ));
                                } else {
                                    response =
                                        ui.add(egui::Button::new("").min_size(
                                            (SQUARE_SIZE + 13.0, SQUARE_SIZE + 13.0).into(),
                                        ));
                                }

                                if response.clicked() {
                                    if self.chess.unit_pos.is_none() {
                                        self.chess.unit_pos =
                                            Some((x_pos as i8, y_pos as i8).into());
                                        println!("{:?}", self.chess.unit_pos);
                                    } else if self.chess.target_pos.is_none() {
                                        self.chess.target_pos =
                                            Some((x_pos as i8, y_pos as i8).into());
                                        println!("{:?}", self.chess.target_pos);
                                    }
                                }
                            }

                            ui.label("");
                            ui.end_row();
                        }
                    });
            });

        self.chess.game_controller();

        egui::CentralPanel::default().show(ctx, |_ui| {});
    }
}
