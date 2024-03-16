use eframe::egui;
use app::transaction;

/// A VopeHist is a table that shows all previous transactions in the Vope
pub struct VopeHist {
    account: Vec<transaction::Transaction>,
}

impl VopeHist {
    pub fn new(data: Vec<transaction::Transaction>) -> Self {
        Self { account: data }
    }
}

impl egui::Widget for VopeHist {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {

        egui::Grid::new("vope_view")
        .num_columns(3)
        .min_col_width(ui.available_width() / 4.0 - 10.0)
        .min_row_height(40.0)
        .striped(true)
        .show(ui, |ui| {

            // Header row
            ui.label("Date");
            ui.label("Description");
            ui.label("Transaction");
            ui.end_row();

            for v in self.account.iter() {
                ui.label(v.date.to_string());
                ui.label(&v.desc);
                ui.label(v.charge.to_string());

                ui.end_row();
            }
        }).response
    }
}