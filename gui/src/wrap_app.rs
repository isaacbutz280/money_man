// 3PP import
use eframe::egui;

// Local library import
// module imports
use crate::features;

// Module definitions
mod acc_mgmt;
mod welcome;

// Any screen to be shown must be shown in the Content menu. Basically
// just a custom version of eframe::App. pass a reference to WrapApp
// so I can dynamically add content screens
trait Content {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) -> Option<Box<dyn Content>>;

    fn content_tag(&self) -> String;
}

// The main content screens will be a welcome and account management
/// A wrap app is the entire GUI. It holds

/// The state that we persist (serialize).
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct WrapApp {
    menu: Box<features::Menu>,   // The top bar
    disp: Box<dyn Content>, // The content to display
}

// The behavior to run every update cycle
impl eframe::App for WrapApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        // Show the top menu bar
        self.menu.update(ctx, frame);

        // Show content screen
        if let Some(c) = self.disp.update(ctx, frame) {
            self.disp = c; 
        }
    }
}

impl WrapApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        #[cfg(feature = "persistence")]
        if let Some(storage) = _cc.storage {
            if let Some(state) = eframe::get_value(storage, eframe::APP_KEY) {
                slf.state = state;
            }
        }

        // Note, we start with just the welcome screen. Once we resolve the
        // account and open, we can add that to the list
        WrapApp {
            menu: Box::new(features::Menu::default()),
            disp: Box::new(welcome::Welcome::new()),
        }
    }
}
