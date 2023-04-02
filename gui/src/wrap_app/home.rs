use eframe::egui;
use crate::features::{piechart, welcome_msg};

#[derive(Default)]
pub struct Home;

impl super::App for Home {
    fn get_display_name(&self) -> String {
        "Home".to_string()
    }
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, acc: &mut app::Account) {

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |columns| {
                columns[0].with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.add_space(ui.available_height() / 3.0);
                    ui.add(welcome_msg::WelcomeMsg::new(
                        acc.name.clone(),
                        acc.date.clone(),
                    ));
                });

                let t = acc.port.accounts
                    .iter()
                    .map(|d| (d.name.as_str(), d.budget.amount as f64))
                    .collect::<Vec<(&str, f64)>>();
                columns[1].add(piechart::PieChart::new(&t));
            });
        });

    }
}
