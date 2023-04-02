use crate::features::vope_hist;
use app::misc;
use eframe::egui::{self, Vec2};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct VopeMgr {
    add_open: bool,
    add_name: String,
    add_budget: String,

    trans_open: bool,
    trans_from: String,
    trans_to: String,
    trans_amount: String,

    edit_open: bool,
    edit_name: String,
    edit_budget: String,
    edit_amount: String,
}

impl Default for VopeMgr {
    fn default() -> Self {
        Self::new()
    }
}

impl VopeMgr {
    fn new() -> Self {
        Self {
            add_open: false,
            trans_open: false,
            edit_open: false,
            add_name: String::default(),
            add_budget: String::default(),
            trans_from: String::default(),
            trans_to: String::default(),
            trans_amount: String::default(),
            edit_name: String::default(),
            edit_budget: String::default(),
            edit_amount: String::default(),
        }
    }
}

impl super::App for VopeMgr {
    fn get_display_name(&self) -> String {
        "Vope Mgr".to_string()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, acc: &mut app::Account) {
        let Self {
            add_open,
            trans_open,
            edit_open,
            add_name,
            add_budget,
            trans_from,
            trans_to,
            trans_amount,
            edit_name,
            edit_budget,
            edit_amount,
        } = self;

        egui::TopBottomPanel::top("vope_mgr_header").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Vope Mgr");
            });
        });

        egui::TopBottomPanel::bottom("acc_history")
            .resizable(true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Vope History");
                    ui.separator();
                });

                egui::ScrollArea::new([false, true]).show(ui, |scroll_ui| {
                    scroll_ui.add(vope_hist::VopeHist::new(acc.get_vope_history()))
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("Select a vope:");

                if ui.button("+").clicked() {
                    *add_open = true;
                }
                ui.add_space(10.0);

                if ui.button("Transfer").clicked() {
                    *trans_open = true;
                }
                ui.add_space(10.0);

                if ui.button("Edit").clicked() {
                    *edit_open = true;
                }
            });

            let mut add_temp = *add_open;

            egui::Window::new("Add").open(add_open).show(ctx, |ui| {
                egui::Grid::new("add_grid")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Vope Name:");
                        ui.add(egui::Separator::default().vertical());
                        ui.add(egui::TextEdit::singleline(add_name));
                        ui.end_row();

                        ui.label("Budgeted:");
                        ui.add(egui::Separator::default().vertical());
                        ui.add(egui::TextEdit::singleline(add_budget));
                        ui.end_row();
                    });

                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button("save").clicked()
                        && acc
                            .add_vope(add_name.as_str(), misc::Dollar::from(add_budget.as_str()))
                            .is_ok()
                    {
                        add_temp = false;
                    }

                    if ui.button("cancel").clicked() {
                        add_temp = false;
                    }
                });
            });

            *add_open &= add_temp;

            let mut trans_temp = *trans_open;

            egui::Window::new("Transfer")
                .open(trans_open)
                .show(ctx, |ui| {
                    egui::Grid::new("trans_grid")
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("From:");
                            ui.add(egui::Separator::default().vertical());
                            ui.add(egui::TextEdit::singleline(trans_from));
                            ui.end_row();

                            ui.label("To:");
                            ui.add(egui::Separator::default().vertical());
                            ui.add(egui::TextEdit::singleline(trans_to));
                            ui.end_row();

                            ui.label("Amount:");
                            ui.add(egui::Separator::default().vertical());
                            ui.add(egui::TextEdit::singleline(trans_amount));
                            ui.end_row();
                        });

                    ui.separator();

                    ui.horizontal(|ui| {
                        if ui.button("save").clicked()
                            && acc
                                .transfer(&trans_from, &trans_to, misc::Dollar::from(trans_amount.as_str()))
                                .is_ok()
                        {
                            // Save state here....
                            trans_temp = false;
                        }

                        if ui.button("cancel").clicked() {
                            trans_temp = false;
                        }
                    });
                });

            *trans_open &= trans_temp;

            let mut edit_temp = *edit_open;

            egui::Window::new("Edit").open(edit_open).show(ctx, |ui| {
                egui::Grid::new("edit_grid")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Name:");
                        ui.add(egui::Separator::default().vertical());
                        ui.add(egui::TextEdit::singleline(edit_name));
                        ui.end_row();

                        ui.label("New Budget:");
                        ui.add(egui::Separator::default().vertical());
                        ui.add(egui::TextEdit::singleline(edit_budget));
                        ui.end_row();

                        ui.label("Set new amount:");
                        ui.add(egui::Separator::default().vertical());
                        ui.add(egui::TextEdit::singleline(edit_amount));
                        ui.end_row();
                    });

                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button("save").clicked() {
                        let r1 = acc.rm_vope(&edit_name);
                        let r2 = acc.add_vope(edit_name, misc::Dollar::from(edit_budget.as_str()));

                        if r1.is_ok() && r2.is_ok() {
                            // Save state here....
                            edit_temp = false;
                        }
                    }

                    if ui.button("cancel").clicked() {
                        edit_temp = false;
                    }

                    if ui.button("Delete").clicked() && acc.rm_vope(&edit_name).is_ok() {
                        // Delete the vope here
                        edit_temp = false;
                    }
                });
            });

            *edit_open &= edit_temp;

            ui.separator();

            let len = acc.port.accounts.len();

            let min_width = ui.available_width() / (len as f32).sqrt().ceil();

            ui.horizontal_wrapped(|ui| {
                for v in acc.port.accounts.iter() {
                    if ui
                        .add(
                            egui::Button::new(v.name.as_str())
                                .min_size(Vec2::new(min_width - 10.0, min_width / 2.0)),
                        )
                        .clicked()
                    {}
                }
            });
        });
    }
}
