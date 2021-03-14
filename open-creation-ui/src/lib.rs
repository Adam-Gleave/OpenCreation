pub mod about_window;
pub mod log_window;
pub mod game_settings_window;

pub use about_window::AboutWindow;
pub use game_settings_window::GameSettingsWindow;
pub use log_window::LogWindow;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub trait Window {
    fn name(&self) -> &'static str;

    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool);
}
