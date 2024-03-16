use directories;
use eframe::egui;
use std::path;

use app;

use super::{acc_mgmt, Content};

/// Wraps many demo/test apps into one.
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
    
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) -> Option<Box<dyn Content>>{
        let mut res: Option<Box<dyn Content>> = None;

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.vertical_centered(|ui| {
                ui.label("Hello");
                
                // Return this result
                let op_acc = if ui.button("Open").clicked() {
                    // Default account location is
                    // %USERPROFILE%\AppData\Roaming\ButzIndustries\MoneyMan\data\acc.json
                    let binding =
                        directories::ProjectDirs::from("io", "ButzIndustries", "MoneyMan").unwrap();
                    let path = path::Path::new(binding.data_dir()).join("acc.json");
                                   
                    Some(Box::new(app::Account::open(path).unwrap()))
                } else if ui.button("Open from...").clicked() {
                    // Do a browser.....
                    todo!()
                } else if ui.button("New").clicked() {
                    Some(Box::new(app::Account::new().unwrap()))
                } else {
                    None
                };

                // If an account was opened, open the 
                if let Some(acc) = op_acc {
                    res = Some(Box::new(acc_mgmt::AccMgmt::new(acc)));
                };

            })
        });

        res
    }
}
