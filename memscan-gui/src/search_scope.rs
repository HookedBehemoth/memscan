use std::fmt::Display;

use egui::ComboBox;
use procfs::process::{MMPermissions, MMapPath, MemoryMap};

#[derive(Debug, PartialEq, Default, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum SearchScope {
    Stack,
    Heap,
    #[default]
    Both,
    All,
}

impl Display for SearchScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchScope::Stack => write!(f, "Stack"),
            SearchScope::Heap => write!(f, "Heap"),
            SearchScope::Both => write!(f, "Stack & Heap"),
            SearchScope::All => write!(f, "All Writeable"),
        }
    }
}

impl SearchScope {
    pub fn picker_for(&mut self, ui: &mut egui::Ui) {
        ComboBox::from_label("Search Space")
            .selected_text(format!("{}", self))
            .show_ui(ui, |ui| {
                ui.selectable_value(self, Self::Stack, "Stack");
                ui.selectable_value(self, Self::Heap, "Heap");
                ui.selectable_value(self, Self::Both, "Stack & Heap");
                ui.selectable_value(self, Self::All, "All Writeable");
            });
    }

    pub fn is_in_scope(self, map: &MemoryMap) -> bool {
        match self {
            SearchScope::Stack => map.pathname == MMapPath::Stack,
            SearchScope::Heap => map.pathname == MMapPath::Heap,
            SearchScope::Both => map.pathname == MMapPath::Heap || map.pathname == MMapPath::Stack,
            SearchScope::All => map.perms & MMPermissions::READ != MMPermissions::empty(),
        }
    }
}
