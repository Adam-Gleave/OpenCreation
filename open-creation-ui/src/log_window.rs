use super::{View, Window};

use std::collections::VecDeque;

const DEFAULT_WIDTH: f32 = 600.0;
const DEFAULT_HEIGHT: f32 = 400.0;

pub struct LogWindow<'a> {
    lines: &'a VecDeque<String>,
    scroll: bool,
}

impl<'a> LogWindow<'a> {
    pub fn new(lines: &'a VecDeque<String>) -> Self {
        Self { lines, scroll: false }
    }

    pub fn scroll(mut self, scroll: bool) -> Self {
        self.scroll = scroll;
        self
    }
}

impl<'a> View for LogWindow<'a> {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered_justified(|ui| {
            egui::ScrollArea::auto_sized()
                .id_source("log_scroll")
                .show(ui, |ui| {
                    let mut lines = self.lines.iter().fold(String::new(), |mut acc, elem| {
                        acc.push_str(elem);
                        acc.push_str("\n");
                        acc
                    });
                    lines.pop();
        
                    let text = egui::widgets::TextEdit::multiline(&mut lines)
                        .text_style(egui::TextStyle::Monospace)
                        .enabled(false);
    
                    let response = ui.add(text);
                    
                    if self.scroll {
                        response.scroll_to_me(egui::Align::BOTTOM);
                    }
                });
        });
    }
}

impl<'a> Window for LogWindow<'a> {
    fn name(&self) -> &'static str {
        "Log"
    }

    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .default_size([DEFAULT_WIDTH, DEFAULT_HEIGHT])
            .scroll(false)
            .show(ctx, |ui| self.ui(ui));
    }
}
