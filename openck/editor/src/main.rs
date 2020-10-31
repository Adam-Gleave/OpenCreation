mod imgui_util;

use imgui::*;

fn main() {
    let mut system = imgui_util::init("OpenCK");
    let display = system.display.clone();

    system.imgui.style_mut().window_rounding = 0f32;
    system.imgui.io_mut().config_flags |= imgui::ConfigFlags::DOCKING_ENABLE;

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

        imgui::Window::new(im_str!("Tree View"))
            .position([10f32, 10f32 + menu_y], Condition::FirstUseEver)
            .size([400f32, y as f32 - (20f32 + menu_y)], Condition::FirstUseEver)
            .build(&ui, || {});

        imgui::Window::new(im_str!("Scene"))
            .position([420f32, 10f32 + menu_y], Condition::FirstUseEver)
            .size([x as f32 - 430f32, y as f32 - (330f32 + menu_y)], Condition::FirstUseEver)
            .build(&ui, || {});

        imgui::Window::new(im_str!("Scene Objects"))
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
