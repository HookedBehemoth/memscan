use crate::{data_types::DataType, endian::Endianness, search_scope::SearchScope};


#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Settings {
    #[serde(skip)]
    show_settings: bool,
    pub default_search_scope: SearchScope,
    pub default_data_type: DataType,
    pub default_endianness: Endianness,
    pub search_buffer_size: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            show_settings: false,
            default_search_scope: SearchScope::default(),
            default_data_type: DataType::default(),
            default_endianness: Endianness::default(),
            search_buffer_size: 128 * 1024 * 1024,
        }
    }
}

impl Settings {
    pub fn toggle(&mut self) {
        self.show_settings = !self.show_settings;
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new("Settings")
            .resizable(true)
            .collapsible(true)
            .open(&mut self.show_settings)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Theme");
                    egui::widgets::global_theme_preference_buttons(ui);
                });
                ui.label("Defaults");
                self.default_data_type.picker_for(ui);
                self.default_search_scope.picker_for(ui);
                let mut value_string = self.search_buffer_size.to_string();
                if ui.text_edit_singleline(&mut value_string).changed() {
                    if let Ok(parsed) = value_string.parse::<usize>() {
                        self.search_buffer_size = parsed;
                    }
                }
            });
    }
}
