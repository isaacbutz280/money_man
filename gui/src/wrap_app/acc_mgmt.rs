use super::Content;
use crate::features::acc_table;
use eframe::egui;

// submod definitions
mod assign;
mod home;
mod vope_mgr;

// Another similar override to the egui::App
trait AccDisp {
    fn update(
        &mut self,
        ctx: &egui::Context,
        frame: &mut eframe::Frame,
        acc: &mut Box<app::Account>,
    );

    fn disp_name(&self) -> String;
}

pub struct AccMgmt {
    indx: usize,
    disp: Vec<Box<dyn AccDisp>>,
    acc: Box<app::Account>,
}

impl Content for AccMgmt {
    fn update(
        &mut self,
        ctx: &eframe::egui::Context,
        frame: &mut eframe::Frame,
    ) -> Option<Box<dyn Content>> {
        // Left side AccDisp menu
        self.sidepanel_left(ctx, frame);

        // Right Side Portfolio display
        self.sidepanel_right(ctx, frame);

        // Main AccDisp to show
        self.disp[self.indx].update(ctx, frame, &mut self.acc);

        // For now, we will never leave this screen
        None
    }

    fn content_tag(&self) -> String {
        "acc_mgmt".to_string()
    }
}

impl AccMgmt {
    pub fn new(acc: Box<app::Account>) -> AccMgmt {
        Self {
            indx: 0,
            disp: vec![
                Box::<home::Home>::default(),
                Box::<assign::Assign>::default(),
                Box::<vope_mgr::VopeMgr>::default(),
            ],
            acc,
        }
    }

    fn sidepanel_left(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("sidepanel_left")
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("💻 Sidebar");
                });

                ui.separator();

                let mut new_ind = self.indx;

                for (i, d) in self.disp.iter().enumerate() {
                    if ui.selectable_label(self.indx == i, d.disp_name()).clicked() {
                        new_ind = i;
                    }
                }

                self.indx = new_ind;

                ui.separator();
            });
    }

    fn sidepanel_right(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("sidepanel_right")
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|head_ui| {
                    head_ui.heading("Account Information");
                });

                ui.separator();

                let p = self.acc.get_portfolio();

                egui::ScrollArea::new([false, true])
                    .always_show_scroll(false)
                    .show(ui, |scroll_ui| {
                        scroll_ui.vertical_centered(|ui| {
                            ui.label(format!("Net Worth: {}", p.view_holdings()));
                            ui.separator();
                            ui.label(format!("Total Budget: {}", p.view_budgeted()))
                        });
                        scroll_ui.separator();

                        scroll_ui.add(acc_table::VertAccDisp::new(p.view_vopes().clone()));
                    });

                ui.separator();
            });
    }
}
