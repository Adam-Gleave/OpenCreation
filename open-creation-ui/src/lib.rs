pub mod log_window;

pub use log_window::LogWindow;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub trait Window {
    fn name(&self) -> &'static str;

    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool);
}
