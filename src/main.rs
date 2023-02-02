mod chess;

#[cfg(feature = "egui")]
use chess::app::ChessEguiApp;
#[cfg(feature = "notan")]
use chess::app::ChessNotanApp;

#[cfg(feature = "egui")]
fn main() {
    ChessEguiApp::run()
}

#[cfg(feature = "notan")]
fn main() {
    ChessNotanApp::run().unwrap();
}
