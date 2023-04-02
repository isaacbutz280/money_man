use eframe::egui;
use app;

pub struct VopeHist {
    account: Vec<app::Transaction>,
}

impl VopeHist {
    pub fn new(data: Vec<app::Transaction>) -> Self {
        Self { account: data }
    }
}
// self.account.len()
impl egui::Widget for VopeHist {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        egui::Grid::new("vope_view")
        .num_columns(4)
        .min_col_width(ui.available_width() / 4.0 - 10.0)
        .min_row_height(40.0)
        .striped(true)
        .show(ui, |ui| {

            ui.label("Date");
            ui.label("Description");
            ui.label("Credit");
            ui.label("Debit");
            // ui.label("Total");

            ui.end_row();

            for v in self.account.iter() {
                ui.label(v.date.to_string());
                ui.label(&v.desc);

                if v.charge.amount < 0.0 {
                    ui.label("");
                    ui.label(v.charge.amount.to_string());
                } else {
                    ui.label(v.charge.amount.to_string());
                    ui.label("");
                }

                ui.end_row();
            }
        }).response
    }
}