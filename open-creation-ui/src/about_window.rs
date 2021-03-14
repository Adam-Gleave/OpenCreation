use super::{View, Window};

const DEFAULT_WIDTH: f32 = 360.0;
const DEFAULT_HEIGHT: f32 = 120.0;

pub struct AboutWindow {}

impl AboutWindow {
    pub fn new() -> Self {
        Self {}
    }
}

impl View for AboutWindow {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.centered_and_justified(|ui| {
            ui.label("Open Creation v0.0.1");
            ui.hyperlink_to("Find us on GitHub", "https://github.com/Adam-Gleave/OpenCreation");
        });
    }
}

impl Window for AboutWindow {
    fn name(&self) -> &'static str {
        "About"
    }

    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .fixed_size(egui::vec2(DEFAULT_WIDTH, DEFAULT_HEIGHT))
            .scroll(false)
            .collapsible(false)
            .show(ctx, |ui| self.ui(ui));
    }
}
