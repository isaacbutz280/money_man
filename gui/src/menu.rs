use eframe::egui;

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Menu {
    night_mode: bool,
}

impl eframe::App for Menu {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::widgets::global_dark_light_mode_switch(ui);

        ui.separator();
        self.demo_windows.ui(ctx);
    }
}