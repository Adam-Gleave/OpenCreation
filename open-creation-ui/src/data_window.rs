use super::{View, Window};

const DEFAULT_WIDTH: f32 = 600.0;
const DEFAULT_HEIGHT: f32 = 400.0;

pub struct DataWindow {
    data_files: Vec<String>
}

impl DataWindow {
    pub fn new() -> Self {
        Self { data_files: vec![] }
    }

    pub fn add_file(&mut self, filename: String) {
        self.data_files.push(filename);
    }
}

impl View for DataWindow {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.columns(3, |columns| {
            columns[0].with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
                egui::ScrollArea::from_max_height(f32::INFINITY).show(ui, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.label("Data Files");
                        ui.separator();
                    });
    
                    self.data_files.iter().for_each(|entry| {
                        ui.selectable_label(false, entry);
                        ui.separator();
                    });
                });
            
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.columns(2, |columns| {
                        columns[0].add(egui::Button::new("Set Active").enabled(false));
                        columns[1].add(egui::Button::new("Details...").enabled(false));
                    });
                });
            });

            columns[1].vertical_centered_justified(|ui| {
                let mut author_text = "".to_string();
                let mut description_text = "".to_string();

                ui.label("Author");
                ui.separator();
                ui.add(egui::TextEdit::singleline(&mut author_text).enabled(false).hint_text("None"));
                ui.label("Description");
                ui.separator();
                ui.centered_and_justified(|ui| {
                    ui.add(egui::TextEdit::multiline(&mut description_text).enabled(false).hint_text("No description"));
                });
            });

            columns[2].vertical_centered_justified(|ui| {
                ui.label("Parent Masters"); 
                ui.separator();
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Min).with_cross_justify(true), |ui| {
                    ui.columns(2, |columns| {
                        columns[0].button("OK");
                        columns[1].button("Cancel");
                    });
                });
            });
        });
    }
}

impl Window for DataWindow {
    fn name(&self) -> &'static str {
        "Data"
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
