use super::{View, Window};

const DEFAULT_WIDTH: f32 = 500.0;
const DEFAULT_HEIGHT: f32 = 280.0;

pub struct GameSettingsWindow {
    entries: Vec<String>,
}

impl GameSettingsWindow {
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

    pub fn add_entry(&mut self, entry: String) {
        self.entries.push(entry);
    }
}

impl View for GameSettingsWindow {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            self.entries.iter().for_each(|entry| {
                ui.selectable_label(false, entry);
                ui.separator();
            });
        });
    }
}

impl Window for GameSettingsWindow {
    fn name(&self) -> &'static str {
        "Game Settings"
    }

    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .default_size(egui::vec2(DEFAULT_WIDTH, DEFAULT_HEIGHT))
            .scroll(true)
            .collapsible(true)
            .show(ctx, |ui| self.ui(ui));
    }
}
