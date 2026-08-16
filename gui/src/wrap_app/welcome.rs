use egui;
use std::path;

use app;

use super::{Content, acc_mgmt};

pub struct Welcome {}

impl Welcome {
    pub fn new() -> Welcome {
        Welcome {}
    }
}

impl Content for Welcome {
    fn content_tag(&self) -> String {
        "Welcome".to_string()
    }

    fn content_ui(
        &mut self,
        ui: &mut egui::Ui,
        frame: &mut eframe::Frame,
    ) -> Option<Box<dyn Content>> {
        let mut res: Option<Box<dyn Content>> = None;

        egui::CentralPanel::no_frame().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("Hello");

                // Return this result
                let op_acc = if ui.button("Open").clicked() {
                    let path = path::PathBuf::new();

                    Some(Box::new(app::Account::open(path).unwrap()))
                } else if ui.button("Open from...").clicked() {
                    // Do a browser.....
                    todo!()
                } else if ui.button("New").clicked() {
                    Some(Box::new(app::Account::new().unwrap()))
                } else {
                    None
                };

                // If an account was opened, go into acc manager
                if let Some(acc) = op_acc {
                    res = Some(Box::new(acc_mgmt::AccMgmt::new(acc)));
                };
            })
        });

        res
    }
}
