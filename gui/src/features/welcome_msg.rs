use eframe::{egui::{Widget, Label, RichText}, epaint::Color32};

pub struct WelcomeMsg {
    name: String,
    date: String,
}

impl WelcomeMsg {
    pub fn new(name: String, date: String) -> Self {
        WelcomeMsg { name, date }
    }
}

impl Widget for WelcomeMsg {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        ui.add(Label::new(RichText::new(format!("Welcome {}", self.name)).size(30.0).color(Color32::from_rgb(200, 0, 200))));
        ui.add(Label::new(RichText::new(format!("The last time you checked accounts was {}", self.date)).size(20.0).color(Color32::BLUE)))
    }
}
