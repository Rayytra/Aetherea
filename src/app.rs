use eframe::*;
use egui::{menu, CentralPanel, TopBottomPanel};

pub struct AethereaApp;

impl eframe::App for AethereaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |_ui| {
            TopBottomPanel::top("menubar").show(ctx, |ui| {
                menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("New").clicked() {
                            println!("New File created.")
                        }

                        if ui.button("Open").clicked() {
                            println!("Opened File.")
                        }

                        ui.menu_button("Open Recent", |ui| {
                            ui.button("randomfile.aeth")
                        });

                        if ui.button("Save").clicked() {
                            println!("Saved.")
                        }

                        if ui.button("Save As").clicked() {
                            println!("Saved As ...")
                        }
                    });

                    ui.menu_button("Edit", |ui| {
                        if ui.button("Undo").clicked() {
                            println!("Undone.")
                        }

                        if ui.button("Redo").clicked() {
                            println!("Redone.")
                        }

                        if ui.button("Preferences").clicked() {
                            println!("Opening Preferences.")
                        }
                    });

                    ui.menu_button("View", |ui| {
                        if ui.button("Reset").clicked() {
                            println!("Reset.")
                        }
                    });
                })
            })
        });
    }
}