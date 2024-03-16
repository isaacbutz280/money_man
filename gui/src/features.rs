use eframe::egui;

pub mod piechart;
pub mod acc_table;
pub mod welcome_msg;
pub mod vope_hist;
mod menu_bar;
pub use menu_bar::Menu;


/// Something to view in the demo windows
pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

/// Something to view
pub trait Feature {
    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui);
}