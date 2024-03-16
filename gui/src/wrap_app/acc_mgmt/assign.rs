use std::collections;

use app::{misc, transaction};
use eframe::{egui, epaint};
use native_dialog::FileDialog;

pub struct Assign {
    cator: Vec<transaction::Transaction>,
    act_t: Option<transaction::Transaction>,
    vope_list: collections::HashMap<String, (bool, f32)>, // I want all vopes, and if they are on or not
    even_weight: bool,
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
            vope_list: collections::HashMap::default(),
            even_weight: false,
        }
    }

    fn update_top_panel(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        acc: &mut app::Account,
    ) {
        egui::TopBottomPanel::top("assign").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Assign");
            });
        });
    }

    fn update_bottom_panel(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        acc: &mut app::Account,
    ) {
        egui::TopBottomPanel::bottom("disp").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.add(egui::Button::new("Ignore")).clicked() {
                    // Deselect all
                    for (b, f) in self.vope_list.values_mut() {
                        *b = false;
                    }
                    // Then we set ignore to true
                    self.vope_list.insert("Ignore".to_string(), (true, 0.0));
                }
                ui.separator();

                ui.checkbox(&mut self.even_weight, "Divide evenly");

                ui.separator();

                let l = self
                    .vope_list
                    .iter()
                    .filter_map(|(k, v)| if v.0 { Some(k.as_str()) } else { None })
                    .collect::<Vec<&str>>()
                    .join(": ");

                let substring: String = l.chars().take(50).collect();

                ui.add_sized(
                    epaint::Vec2::new(
                        ui.available_size_before_wrap().x * 0.5,
                        ui.available_size_before_wrap().y,
                    ),
                    egui::Label::new(substring),
                );
                ui.separator();

                if ui.add(egui::Button::new("->")).clicked() {
                    // If we have a transaction to process...
                    if let Some(transaction) = &self.act_t {
                        // Get all the selected vopes
                        let t: Vec<(&str, f32)> = self
                            .vope_list
                            .iter()
                            .filter_map(|(k, v)| {
                                if v.0 == true {
                                    if self.even_weight {
                                        Some((k.as_str(), 1.0))

                                    } else {
                                        Some((k.as_str(), v.1))
                                    }
                                } else {
                                    None
                                }
                            })
                            .collect();

                        // Send for processing
                        match acc.get_portfolio_mut().assign_transaction(&t, transaction, self.even_weight) {
                            Ok(_) => {
                                // and get the next one
                                self.act_t = self.cator.pop();
                                acc.save();
                            },
                            Err(e) => println!("AHHHHHHHHHH: {}", e),
                        }
                    }
                }
            });
        });
    }

    fn update_center_panel(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        acc: &mut app::Account,
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("Start").clicked() && self.act_t.is_none() {
                    let path = FileDialog::new()
                        .set_location("~/Desktop")
                        .add_filter("CSV File", &["csv"])
                        .show_open_single_file()
                        .unwrap();

                    let path = match path {
                        Some(path) => path,
                        None => return,
                    };

                    match transaction::parse_transactions(&path) {
                        Ok(mut l) => {
                            acc.get_portfolio().clean_transaction_list(&mut l);
                            self.cator.append(&mut l);
                            self.act_t = self.cator.pop();
                        }
                        Err(_) => todo!(),
                    }
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

            let len = acc.get_portfolio().view_vopes().len();

            let min_width = ui.available_width() / (len as f32).sqrt().ceil();

            ui.horizontal_wrapped(|ui| {
                for v in acc.get_portfolio().view_vopes().iter() {
                    let (b, _) = self.vope_list.get_mut(&v.name).unwrap();
                    let color = if *b {
                        egui::Color32::DARK_GREEN
                    } else {
                        egui::Color32::LIGHT_GRAY
                    };

                    if ui
                        .add(
                            egui::Button::new(v.name.as_str())
                                .min_size(epaint::Vec2::new(min_width - 10.0, min_width / 2.0))
                                .fill(color),
                        )
                        .clicked()
                    {
                        *b = !*b;
                        // Turn off ignore any time we click
                        self.vope_list.insert("Ignore".to_string(), (false, 0.0));
                    }
                }
            });
        });
    }
}

impl super::AccDisp for Assign {
    fn disp_name(&self) -> String {
        "Assign".to_string()
    }

    fn update(
        &mut self,
        ctx: &egui::Context,
        frame: &mut eframe::Frame,
        acc: &mut Box<app::Account>,
    ) {
        // If any new vopes, add them in
        for v in acc.get_portfolio().view_vopes() {
            if !self.vope_list.contains_key(&v.name) {
               self.vope_list.insert(v.name.clone(), (false, v.budget.as_f32()));
            }
        }

        self.update_top_panel(ctx, frame, acc);
        self.update_bottom_panel(ctx, frame, acc);
        self.update_center_panel(ctx, frame, acc);
    }
}
