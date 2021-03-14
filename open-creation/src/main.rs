use std::{borrow::{Borrow, BorrowMut}, collections::VecDeque, fs::File, io::Write, sync::{mpsc::{Receiver, sync_channel}, Mutex}};

use bevy::{prelude::*, tasks::{self, Task, TaskPool, TaskPoolBuilder, prelude::*}};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use log::LevelFilter;
use tes_parse::{Plugin, read_plugin};
use lazy_static::lazy_static;

struct PluginResource(Vec<Plugin>);

const LOG_CAPACITY: usize = 1000;

lazy_static! {
    static ref LOGGER: Logger = Logger::new();
}

struct Logger {
    level: Mutex<log::LevelFilter>,
    lines: Mutex<VecDeque<String>>,
}

impl Logger {
    pub fn new() -> Self {
        Self { 
            level: Mutex::new(log::LevelFilter::Warn),
            lines: Mutex::new(VecDeque::with_capacity(LOG_CAPACITY)),
        }
    }

    pub fn filter(&self, filter: log::LevelFilter) {
        *self.level.lock().unwrap() = filter;
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= *self.level.lock().unwrap()
    }

    fn log(&self, record: &log::Record) {
        if let Some(module) = record.module_path() {
            if module.to_owned().contains("tes_parse") {
                let mut lines = self.lines.lock().unwrap();

                if lines.len() > LOG_CAPACITY {
                    lines.pop_front();
                }

                lines.push_back(format!("{}", record.args()));
            }
        }
    }
    
    fn flush(&self) {}
}

struct UiState {
    pub show_log: bool,
}

impl UiState {
    fn new() -> Self {
        Self { show_log: false }
    }
}

fn main() {
    log::set_logger(&*LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    LOGGER.filter(log::LevelFilter::Debug);

    log::warn!("Here");

    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_resource(UiState::new())
        .add_resource(PluginResource(vec![]))
        .add_startup_system(setup.system())
        .add_system(top_panel.system())
        .add_system(left_panel.system())
        .add_system(load_plugin.system())
        .run();
}

fn setup(mut windows: ResMut<Windows>, mut egui_context: ResMut<EguiContext>) {
    windows.get_primary_mut().unwrap().set_title(String::from("Open Creation"));

    let ctx = &mut egui_context.ctx;
    let mut style = (*ctx.style()).clone();
    style.visuals.window_shadow.extrusion = 2.0;
    ctx.set_style(style);
}

fn top_panel(mut egui_ctx: ResMut<EguiContext>, mut ui_state: ResMut<UiState>) {
    let ctx = &mut egui_ctx.ctx;

    egui::TopPanel::top("top_panel").show(ctx, |ui| {
        if ui.selectable_label(ui_state.show_log, "Show Log").clicked() {
            ui_state.show_log = !ui_state.show_log;
        }

        if ui_state.show_log {
            egui::Window::new("Log").default_size([400f32, 200f32]).show(ctx, |ui| {
                egui::ScrollArea::auto_sized().id_source("scroll").show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        let mut lines = LOGGER.lines.lock().unwrap().iter().fold(String::new(), |mut acc, elem| {
                            acc.push_str(elem);
                            acc.push_str("\n");
                            acc
                        });

                        ui.add(egui::widgets::TextEdit::multiline(&mut lines).text_style(egui::TextStyle::Monospace));
                    });
                });
            });
        }
    });
}

fn left_panel(mut egui_ctx: ResMut<EguiContext>) {
    let ctx = &mut egui_ctx.ctx;

    egui::SidePanel::left("side_panel", 800f32).show(ctx, |ui| {
        ui.label("Hello left panel");
    });
}

fn load_plugin(mut plugins: ResMut<PluginResource>) {
    if plugins.0.is_empty() {
        let plugin = read_plugin(File::open("/Users/adam/dev/OpenCreation/data/Skyrim.esm").unwrap()).expect("Help");
        plugins.0.push(plugin);
    }
}