use std::fs::File;

use open_creation_ui::{AboutWindow, LogWindow, Window};
use open_creation_util::{log, Logger};

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use lazy_static::lazy_static;
use tes_parse::{read_plugin, Plugin};

mod ui_state;

lazy_static! {
    static ref LOGGER: Logger = Logger::new();
}

struct PluginResource(Vec<Plugin>);

fn main() {
    log::set_logger(&*LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    LOGGER.filter(log::LevelFilter::Debug);

    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_resource(ui_state::State::new())
        .add_resource(PluginResource(vec![]))
        .add_startup_system(setup.system())
        .add_system(top_panel.system())
        .add_system(left_panel.system())
        .add_system(windows.system())
        .add_system(load_plugin.system())
        .run();
}

fn setup(mut windows: ResMut<Windows>, mut egui_context: ResMut<EguiContext>) {
    windows
        .get_primary_mut()
        .unwrap()
        .set_title(String::from("Open Creation"));

    let ctx = &mut egui_context.ctx;
    let mut style = (*ctx.style()).clone();
    style.visuals.window_shadow.extrusion = 2.0;
    ctx.set_style(style);
}

fn top_panel(mut egui_ctx: ResMut<EguiContext>, mut ui_state: ResMut<ui_state::State>) {
    const MENU_WIDTH: f32 = 150.0;

    let ctx = &mut egui_ctx.ctx;

    let menu_button = |ui: &mut egui::Ui, name: &str| {
        ui.set_width(MENU_WIDTH);
        ui.button(name)
    };

    egui::TopPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            egui::menu::menu(ui, "View", |ui| {
                if menu_button(ui, "Show log").clicked() {
                    ui_state.show_log = !ui_state.show_log;
                }
            });

            egui::menu::menu(ui, "Help", |ui| {
                if menu_button(ui, "About").clicked() {
                    ui_state.show_about = !ui_state.show_about;
                }
            });
        });
    });
}

fn left_panel(mut egui_ctx: ResMut<EguiContext>) {
    let ctx = &mut egui_ctx.ctx;

    egui::SidePanel::left("side_panel", 800f32).show(ctx, |ui| {
        ui.set_max_width(180.0);
        ui.vertical_centered_justified(|ui| {
            if ui.button("Add to log").clicked() {
                log::debug!("Clicked!");
            }
        });
    });
}

fn windows(mut egui_ctx: ResMut<EguiContext>, mut ui_state: ResMut<ui_state::State>) {
    let ctx = &mut egui_ctx.ctx;

    if ui_state.show_about {
        AboutWindow::new().show(ctx, &mut ui_state.show_about);
    }

    if ui_state.show_log {
        let lines = &*LOGGER.lines.lock().unwrap();

        LogWindow::new(lines)
            .scroll(LOGGER.updated())
            .show(ctx, &mut ui_state.show_log);

        LOGGER.set_updated(false);
    }
}

fn load_plugin(mut plugins: ResMut<PluginResource>) {
    if plugins.0.is_empty() {
        let path = std::path::Path::new("/Users/adam/dev/OpenCreation/data/Skyrim.esm");
        let file = File::open(&path);

        if let Ok(file) = file {
            if let Ok(plugin) = read_plugin(file) {
                plugins.0.push(plugin);
            } else {
                log::error!("Error parsing file {}", path.to_string_lossy());
            }
        } else {
            log::error!("Error opening file {}", path.to_string_lossy());
        }
    }
}
