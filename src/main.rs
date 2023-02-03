#[allow(dead_code)]
#[allow(unused_imports)]
mod chess;

#[cfg(feature = "egui")]
use chess::app::egui::ChessEguiApp;
#[cfg(feature = "notan")]
use chess::app::notan::ChessState;

#[cfg(feature = "egui")]
fn main() {
    ChessEguiApp::run()
}

#[cfg(feature = "notan")]
fn main() -> Result<(), String> {
    ChessState::run()
}
