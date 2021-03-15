use std::{borrow::Borrow, default::Default, fs::File};

use open_creation_ui::{AboutWindow, DataWindow, GameSettingsWindow, LogWindow, Window};
use open_creation_util::{log, Logger};

use bevy::{prelude::*, render::camera::PerspectiveProjection, window};
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
        .add_resource(ClearColor(Color::rgb(0.65, 0.65, 0.65)))
        .add_startup_system(setup.system())
        .add_system(should_close.system())
        .add_system(top_panel.system())
        .add_system(left_panel.system())
        .add_system(windows.system())
        .add_system(load_plugin.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    mut windows: ResMut<Windows>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut egui_context: ResMut<EguiContext>,
) {
    let mut window = windows.get_primary_mut().unwrap();
    window.set_maximized(true);
    window.set_title(String::from("Open Creation"));

    let ctx = &mut egui_context.ctx;
    let mut style = (*ctx.style()).clone();
    style.visuals = egui::style::Visuals::light();
    style.visuals.window_corner_radius = 0.0;
    style.visuals.widgets.active.corner_radius = 0.0;
    style.visuals.widgets.hovered.corner_radius = 0.0;
    style.visuals.widgets.noninteractive.corner_radius = 0.0;
    style.visuals.widgets.inactive.corner_radius = 0.0;
    style.visuals.window_shadow.extrusion = 10.0;
    ctx.set_style(style);

    commands.spawn(bevy::prelude::PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::YELLOW_GREEN.into()),
        ..Default::default()
    })
    .spawn(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    })
    .spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(-2.0, 2.5, 5.0)).looking_at(Vec3::default(), Vec3::unit_y()),
        perspective_projection: PerspectiveProjection {
            near: 0.01,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn should_close(mut exit_events: ResMut<Events<bevy::app::AppExit>>, ui_state: Res<ui_state::State>) {
    if ui_state.should_close {
        exit_events.send(bevy::app::AppExit);
    }
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
            egui::menu::menu(ui, "File", |ui| {
                if menu_button(ui, "Data").clicked() {
                    ui_state.show_data = !ui_state.show_data;
                };
                
                if menu_button(ui, "Close").clicked() {
                    ui_state.should_close = true;
                }
            });

            egui::menu::menu(ui, "View", |ui| {
                if menu_button(ui, "Show log").clicked() {
                    ui_state.show_log = !ui_state.show_log;
                }
            });

            egui::menu::menu(ui, "Gameplay", |ui| {
                if menu_button(ui, "Game Settings").clicked() {
                    ui_state.show_game_settings = !ui_state.show_game_settings;
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

fn left_panel(mut egui_ctx: ResMut<EguiContext>, plugins: Res<PluginResource>) {
    let ctx = &mut egui_ctx.ctx;

    egui::SidePanel::left("side_panel", 360f32).show(ctx, |ui| {
        egui::ScrollArea::auto_sized().show(ui, |ui| {
            ui.separator();
            ui.vertical_centered_justified(|ui| {
                ui.label("Tree View");
            });
            ui.separator();

            let plugins = &plugins.borrow().0;

            let populate_by_code = |ui: &mut egui::Ui, code: [u8; 4]| {
                ui.with_layout(egui::Layout::top_down(egui::Align::TOP).with_cross_justify(true), |ui| {
                    ui.separator();

                    for plugin in plugins {
                        for editor_id in plugin.get_editor_ids_by_code(code) {
                            ui.selectable_label(false, editor_id);
                            ui.separator();
                        }
                    }
                });
            };

            let node = |ui: &mut egui::Ui, name: &str, code: [u8; 4]| {
                ui.collapsing(name, |ui| {
                    populate_by_code(ui, code);
                });
            };

            ui.collapsing("Actors", |ui| {
                populate_by_code(ui, [b'N', b'P', b'C', b'_']);
            });

            ui.collapsing("Audio", |ui| {
                node(ui, "Acoustic Space",          [b'A', b'S', b'P', b'C']);
                node(ui, "Music Track",             [b'M', b'U', b'S', b'T']);
                node(ui, "Music Type",              [b'M', b'U', b'S', b'C']);
                node(ui, "Reverb",                  [b'R', b'E', b'V', b'B']);
                node(ui, "Sound Category",          [b'S', b'N', b'C', b'T']);
                node(ui, "Sound Marker",            [b'S', b'O', b'U', b'N']);
                node(ui, "Sound Output Model",      [b'S', b'O', b'P', b'M']);
            });

            ui.collapsing("Character", |ui| {
                node(ui, "AI Package",              [b'P', b'A', b'C', b'K']);
                node(ui, "Association Type",        [b'A', b'S', b'T', b'P']);
                node(ui, "Class",                   [b'C', b'L', b'A', b'S']);
                node(ui, "Equip Slot",              [b'E', b'Q', b'U', b'P']);
                node(ui, "Faction",                 [b'F', b'A', b'C', b'T']);
                node(ui, "Head Part",               [b'H', b'D', b'P', b'T']);
                node(ui, "Movement Type",           [b'M', b'O', b'V', b'T']);
                node(ui, "Quest",                   [b'Q', b'U', b'S', b'T']);
                node(ui, "Race",                    [b'R', b'A', b'C', b'E']);
                node(ui, "Relationship",            [b'R', b'E', b'L', b'A']);
                node(ui, "Story Manager Event",     [b'S', b'M', b'E', b'N']);
                node(ui, "Voice Type",              [b'V', b'T', b'Y', b'P']);
            });

            ui.collapsing("Items", |ui| {
                node(ui, "Ammo",                    [b'A', b'M', b'M', b'O']);
                node(ui, "Armor",                   [b'A', b'R', b'M', b'O']);
                node(ui, "Book",                    [b'B', b'O', b'O', b'K']);
                node(ui, "Constructible Object",    [b'C', b'O', b'B', b'J']);
                node(ui, "Ingredient",              [b'I', b'N', b'G', b'R']);
                node(ui, "Key",                     [b'K', b'E', b'Y', b'M']);
                node(ui, "Leveled Item",            [b'L', b'V', b'L', b'I']);
                node(ui, "Misc Item",               [b'M', b'I', b'S', b'C']);
                node(ui, "Outfit",                  [b'O', b'T', b'F', b'T']);
                node(ui, "Soul Gem",                [b'S', b'L', b'G', b'M']);
                node(ui, "Weapon",                  [b'W', b'E', b'A', b'P']);
                node(ui, "Quest",                   [b'Q', b'U', b'S', b'T']);
            });

            ui.collapsing("Magic", |ui| {});
            
            ui.collapsing("Miscellaneous", |ui| {
                node(ui, "Animation Object",        [b'A', b'N', b'I', b'O']);
                node(ui, "Art Object",              [b'A', b'R', b'T', b'O']);
                node(ui, "Collision Layer",         [b'C', b'O', b'L', b'L']);
                node(ui, "Color Form",              [b'C', b'L', b'F', b'M']);
                node(ui, "Combat Style",            [b'C', b'S', b'T', b'Y']);
                node(ui, "Form List",               [b'F', b'L', b'S', b'T']);
                node(ui, "Global",                  [b'G', b'L', b'O', b'B']);
                node(ui, "Idle Marker",             [b'I', b'D', b'L', b'M']);
                node(ui, "Keyword",                 [b'K', b'Y', b'W', b'D']);
                node(ui, "Land Texture",            [b'L', b'T', b'E', b'X']);
                node(ui, "Load Screen",             [b'L', b'S', b'C', b'R']);
                node(ui, "Material Object",         [b'M', b'A', b'T', b'O']);
                node(ui, "Message",                 [b'M', b'E', b'S', b'G']);
                node(ui, "Texture Set",             [b'T', b'X', b'S', b'T']);
            });

            ui.collapsing("Special Effects", |ui| {});
            ui.collapsing("World Data", |ui| {});

            ui.collapsing("World Objects", |ui| {
                node(ui, "Activator",               [b'A', b'C', b'T', b'I']);
                node(ui, "Container",               [b'C', b'O', b'N', b'T']);
                node(ui, "Door",                    [b'D', b'O', b'O', b'R']);
                node(ui, "Flora",                   [b'F', b'L', b'O', b'R']);
                node(ui, "Furniture",               [b'F', b'U', b'R', b'N']);
                node(ui, "Grass",                   [b'G', b'R', b'A', b'S']);
                node(ui, "Light",                   [b'L', b'I', b'G', b'H']);
                node(ui, "Movable Static",          [b'M', b'S', b'T', b'T']);
                node(ui, "Static",                  [b'S', b'T', b'A', b'T']);
                node(ui, "Tree",                    [b'T', b'R', b'E', b'E']);
            });
        });
    });
}

fn windows(mut egui_ctx: ResMut<EguiContext>, mut ui_state: ResMut<ui_state::State>, plugins: Res<PluginResource>) {
    let ctx = &mut egui_ctx.ctx;

    if ui_state.show_data {
        let mut data_window = DataWindow::new();
        let files = vec!["Skyrim.esm", "Dawnguard.esm", "Dragonborn.esm", "Update.esm"];

        for file in files {
            data_window.add_file(file.to_owned());
        }

        data_window.show(ctx, &mut ui_state.show_data);
    }

    if ui_state.show_about {
        AboutWindow::new().show(ctx, &mut ui_state.show_about);
    }

    if ui_state.show_game_settings {
        let mut game_settings_window = GameSettingsWindow::new();
        let plugins = &plugins.borrow().0;

        for plugin in plugins {
            for editor_id in plugin.get_editor_ids_by_code([b'G', b'M', b'S', b'T']) {
                game_settings_window.add_entry(editor_id);
            }
        }

        game_settings_window.show(ctx, &mut ui_state.show_game_settings);
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
