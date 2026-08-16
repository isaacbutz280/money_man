use eframe::egui;

/**
 * This module contains the menu bar layout and tooling
 */

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Menu {
    night_mode: bool,
}

impl eframe::App for Menu {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::Panel::top("wrap_app_top_bar").show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.separator();
                ui.add(egui::widgets::Checkbox::new(&mut true, "A test box"));
            });
        });
    }
}

