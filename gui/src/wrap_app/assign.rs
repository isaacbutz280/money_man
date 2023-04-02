use eframe::{egui, epaint};
use app;

pub struct Assign {
    cator: Vec<app::Transaction>,
    act_t: Option<app::Transaction>,
    sel_vope: String,
}

impl Default for Assign {
    fn default() -> Self {
        Self::new()
    }
}

impl Assign {
    fn new() -> Self {
        Self {
            cator: Vec::new(),
            act_t: None,
            sel_vope: String::default(),
        }
    }
}

impl super::App for Assign {
    fn get_display_name(&self) -> String {
        "Assign".to_string()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, acc: &mut app::Account) {
        egui::TopBottomPanel::top("assign").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Assign");
            });
        });

        egui::TopBottomPanel::bottom("disp").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.add(egui::Button::new("Salary")).clicked() {
                    self.sel_vope = "Salary".to_string();
                }
                ui.separator();

                if ui.add(egui::Button::new("Ignore")).clicked() {
                    self.sel_vope = "Ignore".to_string();
                }
                ui.separator();

                ui.add_sized(
                    epaint::Vec2::new(
                        ui.available_size_before_wrap().x * 0.5,
                        ui.available_size_before_wrap().y,
                    ),
                    egui::Label::new(self.sel_vope.as_str()),
                );
                ui.separator();

                if ui.add(egui::Button::new("->")).clicked() {
                    // If we have a transaction to process...
                    if let Some(transaction) = &self.act_t { 
                        // Process, and get the next one
                        match acc.update_vope(&self.sel_vope, transaction) {
                            Ok(()) => self.act_t = self.cator.pop(),
                            Err(e) => println!("AHHHHHHHHHH: {}", e),
                        }
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {

                if ui.button("Start").clicked() && self.act_t.is_none() {
                    self.cator.append(&mut acc.get_uncatorgorized());
                    self.act_t = self.cator.pop();
                }
                ui.separator();

                ui.add(egui::Label::new("Which Vope?:"));
                ui.separator();

                ui.label(match &self.act_t {
                    Some(t) => t.date.to_string(),
                    None => "".to_string(),
                });
                ui.separator();

                ui.label(match &self.act_t {
                    Some(t) => t.desc.to_string(),
                    None => "".to_string(),
                });
                ui.separator();

                ui.label(match &self.act_t {
                    Some(t) => t.charge.to_string(),
                    None => "".to_string(),
                });
                ui.separator();
            });

            ui.separator();

            let len = acc.port.accounts.len();

            let min_width = ui.available_width() / (len as f32).sqrt().ceil();

            ui.horizontal_wrapped(|ui| {
                for v in acc.port.accounts.iter() {
                    if ui
                        .add(
                            egui::Button::new(v.name.as_str())
                                .min_size(epaint::Vec2::new(min_width - 10.0, min_width / 2.0)),
                        )
                        .clicked()
                    {
                        self.sel_vope = v.name.clone();
                    }
                }
            });
        });
    }
}
