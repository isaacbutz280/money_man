use app::vope;
use eframe::egui::{Widget, self};

pub struct AccDisp {
    account: Vec<vope::Vope>,
}

impl AccDisp {
    pub fn new(data: Vec<vope::Vope>) -> Self {
        Self { account: data }
    }
}
// self.account.len()
impl Widget for AccDisp {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {

        egui::Grid::new("vope_view")
        .num_columns(3)
        .spacing([40.0, 40.0])
        .striped(true)
        .show(ui, |ui| {

            ui.label("Vope");
            ui.label("Budgeted");
            ui.label("Actual");

            ui.end_row();

            for v in self.account.iter() {
                ui.label(v.name.as_str());
                ui.label(v.budget.to_string());
                ui.label(v.actual_amount.to_string());

                ui.end_row();
            }
        }).response
    }
}