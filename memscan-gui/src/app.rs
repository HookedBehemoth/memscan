use std::collections::HashMap;

use egui::{Align, Layout, ScrollArea, Sense, Window};
use egui_extras::{Column, TableBuilder};

use crate::{
    app_error::AppError,
    data_types::{DataType, WrappedValue},
    endian::Endianness,
    process_picker::ProcessPicker,
    search::{self, SearchRegion, read_value},
    search_scope::SearchScope,
    settings::Settings,
};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    #[serde(skip)]
    search: String,

    #[serde(skip)]
    editor: Option<Editor>,

    #[serde(skip)]
    show_processes: bool,

    #[serde(skip)]
    process_picker: ProcessPicker,

    #[serde(skip)]
    search_regions: Vec<SearchRegion>,

    #[serde(skip)]
    data_type: DataType,

    #[serde(skip)]
    search_scope: SearchScope,

    #[serde(skip)]
    endianness: Endianness,

    #[serde(skip)]
    search_results: Vec<(i32, Vec<u64>)>,

    #[serde(skip)]
    tracked_addresses: HashMap<(i32, u64), WrappedValue>,

    #[serde(skip)]
    error: Option<AppError>,

    settings: Settings,
}

struct Editor {
    pid: i32,
    pointer: u64,
    data_type: DataType,
    endianness: Endianness,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            search: String::new(),
            editor: None,
            show_processes: false,
            process_picker: ProcessPicker::new(),
            search_regions: vec![],
            data_type: DataType::default(),
            search_scope: SearchScope::default(),
            endianness: Endianness::default(),
            search_results: vec![],
            tracked_addresses: HashMap::new(),
            error: None,
            settings: Settings::default(),
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            let mut _self: TemplateApp =
                eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            _self.search_scope = _self.settings.default_search_scope;
            _self.data_type = _self.settings.default_data_type;
            _self.endianness = _self.settings.default_endianness;
            return _self;
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                ui.button("Settings").clicked().then(|| {
                    self.settings.toggle();
                });
            });
        });

        self.settings.show(ctx);

        egui::SidePanel::left("process_picker")
            .resizable(true)
            .default_width(600.0)
            .show(ctx, |ui| {
                ScrollArea::horizontal().show(ui, |ui| {
                    self.process_picker.show(ui);
                });
                // ui.separator();
            });

        egui::SidePanel::right("results")
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Results");
                ui.horizontal(|ui| {
                    ui.label("Count");
                    ui.label(
                        self.search_results
                            .iter()
                            .fold(0usize, |acc, (_, pointers)| acc + pointers.len())
                            .to_string(),
                    );
                });

                TableBuilder::new(ui)
                    .striped(true)
                    .cell_layout(Layout::left_to_right(Align::Center))
                    .column(Column::auto().at_least(50.0))
                    .column(Column::remainder())
                    .sense(Sense::click())
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.label("PID");
                        });
                        header.col(|ui| {
                            ui.label("Results");
                        });
                    })
                    .body(|mut body| {
                        for (pid, pointers) in &self.search_results {
                            for pointer in pointers.iter().take(10) {
                                body.row(20.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(format!("{pid}"));
                                    });
                                    row.col(|ui| {
                                        ui.label(format!("{pointer:x}"));
                                    });

                                    // Rows are selectable
                                    let key = (*pid, *pointer);
                                    row.set_selected(self.tracked_addresses.contains_key(&key));
                                    row.response().clicked().then(|| {
                                        if self.tracked_addresses.contains_key(&key) {
                                            self.tracked_addresses.remove(&key);
                                        } else {
                                            let value =
                                                match read_value(*pid, *pointer, self.data_type) {
                                                    Ok(value) => value,
                                                    Err(error) => {
                                                        eprintln!("Error: {error}");
                                                        self.error = Some(error);
                                                        return;
                                                    }
                                                };
                                            self.tracked_addresses.insert(key, value);
                                        }
                                    });
                                });
                            }
                        }
                    });

                if self.search_results.len() > 100 {
                    ui.horizontal(|ui| {
                        ui.label("... and more");
                    });
                }
                ui.separator();
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Search");

            ui.horizontal(|ui| {
                ui.button("Load Regions").clicked().then(|| {
                    let selected = self.process_picker.selected_processes();
                    self.search_regions = SearchRegion::load(&selected, self.search_scope);
                });
                self.search_scope.picker_for(ui);
            });

            ui.horizontal(|ui| {
                ui.label("Search regions: ");
                ui.label(self.search_regions.len().to_string());
            });

            ui.horizontal(|ui| {
                self.data_type.picker_for(ui);
            });

            ui.horizontal(|ui| {
                ui.label("Value:");
                ui.text_edit_singleline(&mut self.search);
            });
            ui.horizontal(|ui| {
                ui.button("Search").clicked().then(|| {
                    // Begin new search
                    match if self.search_results.is_empty() {
                        search::search_sync(
                            &self.search_regions,
                            self.data_type,
                            &self.search,
                            &self.settings,
                            self.endianness,
                        )
                    }
                    // Scan previous results
                    else {
                        search::search_continue_sync(
                            &self.search_results,
                            self.data_type,
                            &self.search,
                            &self.settings,
                            self.endianness,
                        )
                    } {
                        Ok(results) => {
                            self.search_results = results;
                        }
                        Err(err) => {
                            eprintln!("Error: {err}");
                            self.error = Some(err);
                        }
                    }
                });

                ui.button("Clear").clicked().then(|| {
                    self.search_results.clear();
                });
            });

            ui.separator();

            for region in &self.search_regions {
                ui.horizontal(|ui| {
                    ui.label(format!("PID: {}", region.pid));
                    ui.label(format!("Start: {:#x}", region.start));
                    ui.label(format!("End: {:#x}", region.end));
                    ui.label(format!("Size: {:#x}", region.end - region.start));
                });
            }

            ui.separator();

            ui.button("Refresh").clicked().then(|| {
                let keys: Vec<_> = self.tracked_addresses.keys().cloned().collect();
                for key in keys {
                    if let Ok(value) = read_value(key.0, key.1, self.data_type) {
                        self.tracked_addresses.insert(key, value);
                    } else {
                        self.tracked_addresses.remove(&key);
                    }
                }
            });

            for ((pid, pointer), value) in &self.tracked_addresses {
                ui.horizontal(|ui| {
                    ui.label(format!("PID: {}", pid));
                    ui.label(format!("Pointer: {:#x}", pointer));
                    ui.label(format!("Type: {}", value.data_type()));
                    ui.label(format!("Value: {value}"));
                    ui.button("Edit").clicked().then(|| {
                        self.editor = Some(Editor {
                            pid: *pid,
                            pointer: *pointer,
                            data_type: value.data_type(),
                            endianness: self.endianness,
                        });
                    });
                });
            }

            Window::new("Error")
                .resizable(false)
                .collapsible(false)
                .open(&mut self.error.is_some())
                .show(ctx, |ui| {
                    if let Some(err) = &self.error {
                        ui.label(format!("{err}"));
                    }
                    ui.horizontal(|ui| {
                        if ui.button("OK").clicked() {
                            self.error = None;
                        }
                    });
                });

            Window::new("Editor")
                .resizable(false)
                .collapsible(false)
                .open(&mut self.editor.is_some())
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Value:");
                        ui.text_edit_singleline(&mut self.search);
                    });
                    ui.horizontal(|ui| {
                        ui.button("OK").clicked().then(|| {
                            if let Some(editor) = &self.editor {
                                let value = match editor.data_type.parse(&self.search) {
                                    Ok(value) => value,
                                    Err(error) => {
                                        eprintln!("Error: {error}");
                                        self.error = Some(error);
                                        return;
                                    }
                                };
                                self.tracked_addresses
                                    .insert((editor.pid, editor.pointer), value);
                            }
                            self.editor = None;
                        });
                    });
                });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
