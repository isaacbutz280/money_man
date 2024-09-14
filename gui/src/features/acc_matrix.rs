use eframe::egui;

pub struct AccMatrix {
    buttons: Vec<(String, bool)>,
}

impl AccMatrix {
    pub fn new(buttons: Vec<(String, bool)>) -> Self {
        Self { buttons }
    }
}

impl egui::Widget for AccMatrix {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        for (name, selected) in self.buttons {
            let color = if selected {
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


    }
}
