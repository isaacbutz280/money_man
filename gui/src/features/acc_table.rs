use app::{dollar, vope};
use eframe::egui;

pub struct VertAccDisp {
    accounts: Vec<vope::Vope>,
}

impl VertAccDisp {
    pub fn new(
        accounts: Vec<vope::Vope>,
    ) -> Self {
        Self {
            accounts,
        }
    }

    // fn get_grid(&self) -> egui::Grid {
    //     egui::Grid::new("vope_view")
    //         .num_columns(3)
    //         .spacing([40.0, 40.0])
    //         .striped(true)
    //         .show(ui, |ui| {
    //             ui.label("Vope");
    //             ui.label("Budgeted");
    //             ui.label("Actual");

    //             ui.end_row();

    //             for v in self.account.iter() {
    //                 ui.label(v.name.as_str());
    //                 ui.label(v.budget.to_string());
    //                 ui.label(v.actual_amount.to_string());

    //                 ui.end_row();
    //             }
    //         })
    // }
}
// self.account.len()

/**
 *
 */

impl egui::Widget for VertAccDisp {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        egui::Grid::new("vope_view")
        .num_columns(3)
        .spacing([40.0, 40.0])
        .striped(true)
        .show(ui, |ui| {

            ui.label("Vope");
            ui.label("Budgeted");
            ui.label("Actual");

            ui.end_row();

            for v in self.accounts.iter() {
                ui.label(v.name.as_str());
                ui.label(v.budget.to_string());
                ui.label(v.actual_amount.to_string());

                ui.end_row();
            }
        }).response
    }
}
