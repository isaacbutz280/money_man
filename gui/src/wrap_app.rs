// 3PP import
use egui;

// Local library import
// module imports
use crate::features;

// Module definitions
mod acc_mgmt;
mod welcome;

/// The Content trait is basically just a copy of the eframe::App trait. The major
/// difference is that by returning a Box, I can switch my app screen easily
trait Content {
    fn content_ui(
        &mut self,
        ui: &mut egui::Ui,
        frame: &mut eframe::Frame,
    ) -> Option<Box<dyn Content>>;

    fn content_tag(&self) -> String;
}

// The main content screens will be a welcome and account management
/// A wrap app is the entire GUI. It holds
///
/// The state that we persist (serialize).
pub struct WrapApp {
    menu: Box<features::Menu>, // The top bar
    disp: Box<dyn Content>,    // The content to display
}

// The behavior to run every update cycle
impl eframe::App for WrapApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        // Show the top menu bar
        self.menu.ui(ui, frame);

        // Show content screen
        if let Some(c) = self.disp.content_ui(ui, frame) {
            self.disp = c;
        }
    }
}

impl WrapApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Note, we start with just the welcome screen. Once we resolve the
        // account and open, we can add that to the list
        WrapApp {
            menu: Box::new(features::Menu::default()),
            disp: Box::new(welcome::Welcome::new()),
        }
    }
}
