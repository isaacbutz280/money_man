use app;
use eframe::egui;
use crate::features;

pub mod assign;
pub mod home;
pub mod vope_mgr;

trait App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, acc: &mut app::Account);

    fn get_display_name(&self) -> String;
}

/// The state that we persist (serialize).
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct State {
    selected_anchor: usize,
    disp: Vec<Box<dyn App>>,
    acc: app::Account,
}

impl State {
    fn new(acc: app::Account) -> Self {
        Self {
            selected_anchor: 0,
            disp: vec![
                Box::<home::Home>::default(),
                Box::<assign::Assign>::default(),
                Box::<vope_mgr::VopeMgr>::default(),
            ],
            acc,
        }
    }
}

/// Wraps many demo/test apps into one.
pub struct WrapApp {
    state: State,
}

impl WrapApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, acc: app::Account) -> Self {
        #[allow(unused_mut)]
        let mut slf = Self {
            state: State::new(acc),
        };

        #[cfg(feature = "persistence")]
        if let Some(storage) = _cc.storage {
            if let Some(state) = eframe::get_value(storage, eframe::APP_KEY) {
                slf.state = state;
            }
        }

        slf
    }
}

impl eframe::App for WrapApp {
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.state);
    }

    fn clear_color(&self, visuals: &egui::Visuals) -> egui::Rgba {
        visuals.panel_fill.into()
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if ctx
            .input_mut()
            .consume_key(egui::Modifiers::NONE, egui::Key::F11)
        {
            frame.set_fullscreen(!frame.info().window_info.fullscreen);
        }

        // The top menu bar
        self.menu_contents(ctx, frame);

        // The left menu
        self.sidebar_panel(ctx, frame);

        // The right account display
        self.acc_disp_panel(ctx, frame);

        // And show our page...
        self.state.disp[self.state.selected_anchor].update(ctx, frame, &mut self.state.acc);
    }
}

impl WrapApp {
    fn menu_contents(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("wrap_app_top_bar").show(ctx, |ui| {
            egui::trace!(ui);

            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.separator();
                ui.add(egui::widgets::Checkbox::new(&mut true, "A test box"));
            });
        });
    }

    fn sidebar_panel(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("backend_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("💻 Sidebar");
                });

                ui.separator();

                let mut selected_anchor = self.state.selected_anchor;

                for (ind, app) in self.state.disp.iter().enumerate() {
                    if ui
                        .selectable_label(selected_anchor == ind, app.get_display_name())
                        .clicked()
                    {
                        selected_anchor = ind;

                        // if frame.is_web() {
                        //     ui.output().open_url(format!("#{}", anchor));
                        // }
                    }
                }

                self.state.selected_anchor = selected_anchor;

                ui.separator();
            });
    }

    fn acc_disp_panel(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("acc_disp")
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|head_ui| {
                    head_ui.heading("Account Information");
                });

                ui.separator();

                egui::ScrollArea::new([false, true])
                    .always_show_scroll(false)
                    .show(ui, |scroll_ui| {

                        scroll_ui.vertical_centered(|ui| {
                            ui.label(format!("Net Worth: {}", self.state.acc.port.holdings));
                            ui.separator();
                            ui.label(format!("Total Budget: {}", self.state.acc.port.budgeted))
                        });
                        scroll_ui.separator();

                        scroll_ui.add(features::acc_disp::AccDisp::new(
                            self.state.acc.port.accounts.clone(),
                        ));
                    });
            });
    }
}
