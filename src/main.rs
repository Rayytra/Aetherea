#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app;

use eframe::{run_native, NativeOptions};
use app::AethereaApp;

fn main() -> eframe::Result<(), eframe::Error> {
    let native_options = NativeOptions {
        centered: true,
        ..Default::default()
    };
    
    let _ = run_native(
        "Aetherea",
        native_options,
        Box::new(|_cc| {
            Ok(Box::new(AethereaApp))
        })
    );
    Ok(())
}
