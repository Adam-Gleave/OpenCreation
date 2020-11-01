mod imgui_util;
mod state;
mod tree_view;

use crate::tree_view::TreeView;
use imgui::*;
use std::sync::{Arc, RwLock};

fn main() {
    let ui_state = Arc::new(RwLock::new(state::State::new()));
    let mut system = imgui_util::init("OpenCK");
    let display = system.display.clone();

    system.imgui.style_mut().window_rounding = 0f32;
    system.imgui.io_mut().config_flags |= imgui::ConfigFlags::DOCKING_ENABLE;
    system.imgui.io_mut().config_windows_move_from_title_bar_only = true;

    let mut about_open = false;

    system.main_loop(move |_, ui| {
        if let Some(menu_bar) = ui.begin_main_menu_bar() {
            if let Some(menu) = ui.begin_menu(im_str!("File"), true) {
                MenuItem::new(im_str!("Data...")).build(ui);

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

        let (screen_x, screen_y) = display.get_framebuffer_dimensions();
        let menu_y = 18f32;
        ui_state.write().unwrap().screen_x = screen_x as f32;
        ui_state.write().unwrap().screen_y = screen_y as f32;

        DockSpace::new(im_str!("Dockspace")).over_viewport();

        TreeView::new().build(&ui, Arc::clone(&ui_state));

        Window::new(im_str!("Scene"))
            .position([420f32, 10f32 + menu_y], Condition::FirstUseEver)
            .size(
                [screen_x as f32 - 430f32, screen_y as f32 - (330f32 + menu_y)],
                Condition::FirstUseEver,
            )
            .build(&ui, || {});

        Window::new(im_str!("Cell View"))
            .position([420f32, screen_y as f32 - 310f32], Condition::FirstUseEver)
            .size([screen_x as f32 - 430f32, 300f32], Condition::FirstUseEver)
            .build(&ui, || {});

        if about_open {
            Window::new(im_str!("About"))
                .size([300f32, 100f32], Condition::Appearing)
                .position([screen_x as f32 / 2f32, screen_y as f32 / 2f32], Condition::Appearing)
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
