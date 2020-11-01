mod imgui_util;

use imgui::*;
use session::{Plugin, Record, TypeCode};
use std::ffi::CString;
use std::fs::File;
use std::path::PathBuf;

struct State<'a> {
    plugin: &'a Plugin,
}

fn main() {
    let path = PathBuf::from(format!(
        "{}{}",
        env!("CARGO_MANIFEST_DIR"),
        "/../../data/Skyrim.esm"
    ));
    println!("Path: {:#?}", path);
    let file = File::open(path).unwrap();
    let plugin = Plugin::from_file(file, true).unwrap();

    let mut system = imgui_util::init("OpenCK");
    let display = system.display.clone();

    system.imgui.style_mut().window_rounding = 0f32;
    system.imgui.io_mut().config_flags |= imgui::ConfigFlags::DOCKING_ENABLE;
    system.imgui.io_mut().config_windows_move_from_title_bar_only = true;

    let mut about_open = false;

    system.main_loop( move |_, ui| {
        if let Some(menu_bar) = ui.begin_main_menu_bar() {
            if let Some(menu) = ui.begin_menu(im_str!("File"), true) {
                MenuItem::new(im_str!("Data..."))
                    .build(ui);

                menu.end(ui);
            }
            if let Some(menu) = ui.begin_menu(im_str!("Help"), true) {
                if MenuItem::new(im_str!("About OpenCK")).build(ui) {
                    about_open = true;
                }

                menu.end(ui);
            }
            menu_bar.end(ui);
        }
            
        let (x, y) = display.get_framebuffer_dimensions();
        let menu_y = 18f32;

        DockSpace::new(im_str!("Dockspace")).over_viewport();

        Window::new(im_str!("Object View"))
            .position([10f32, 10f32 + menu_y], Condition::FirstUseEver)
            .size([400f32, y as f32 - (20f32 + menu_y)], Condition::FirstUseEver)
            .build(&ui, || {
                let actors = plugin.get_records_by_code(TypeCode::from_utf8("NPC_").unwrap());

                TreeNode::new(im_str!("TreeNode"))
                    .label(im_str!("Plugin File"))
                    .opened(true, Condition::FirstUseEver)
                    .build(ui, || {
                        TreeNode::new(im_str!("ActorNode"))
                            .label(im_str!("Actors"))
                            .build(ui, || {
                                if let Some(actors) = &actors {
                                    actors.iter().enumerate().for_each(|(pos, r)| {
                                        let id_string = format!("ActorLeaf{}{}", pos, "\0");
                                        let text_string = format!("ID: {:#010x}{}", r.header.id, "\0");
                                        TreeNode::new(unsafe {
                                            ImStr::from_utf8_with_nul_unchecked(id_string.as_bytes())
                                        }).leaf(true)
                                          .label(unsafe { ImStr::from_utf8_with_nul_unchecked(text_string.as_bytes()) })
                                          .build(ui, || {});
                                    });
                                }
                            });
                    });
            });      

        Window::new(im_str!("Scene"))
            .position([420f32, 10f32 + menu_y], Condition::FirstUseEver)
            .size([x as f32 - 430f32, y as f32 - (330f32 + menu_y)], Condition::FirstUseEver)
            .build(&ui, || {});

        Window::new(im_str!("Cell View"))
            .position([420f32, y as f32 - 310f32], Condition::FirstUseEver)
            .size([x as f32 - 430f32, 300f32], Condition::FirstUseEver)
            .build(&ui, || {});

        if about_open {
            Window::new(im_str!("About"))
                .size([300f32, 100f32], Condition::Appearing)
                .position([x as f32 / 2f32, y as f32 / 2f32], Condition::Appearing)
                .position_pivot([0.5f32, 0.5f32])
                .collapsible(false)
                .opened(&mut about_open)
                .build(ui, || {
                    ui.text("OpenCK v0.0.1");
                    ui.text("Welcome to the OpenCK!");
                });
            }
    });
}
