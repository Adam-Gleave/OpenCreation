mod imgui_util;

use imgui::*;

fn main() {
    let mut system = imgui_util::init(file!());

    system.imgui.style_mut().window_rounding = 0f32;
    system.imgui.io_mut().config_flags |= imgui::ConfigFlags::DOCKING_ENABLE;

    let mut first_open = true;
    let mut about_open = true;

    system.main_loop(move |_, ui| {
        DockSpace::new(im_str!("Dockspace"))
            .flags(DockNodeFlags::NO_RESIZE)
            .over_viewport();

        if let Some(menu_bar) = ui.begin_main_menu_bar() {
            if let Some(menu) = ui.begin_menu(im_str!("File"), true) {
                MenuItem::new(im_str!("Data..."))
                    .enabled(true)
                    .build(ui);

                menu.end(ui);
            }
            menu_bar.end(ui);
        }

        if about_open {
            Window::new(im_str!("About"))
                .size([300f32, 100f32], Condition::Appearing)
                .opened(&mut about_open)
                .build(ui, || {
                    ui.text("OpenCK v0.0.1");
                    ui.text("Welcome to the OpenCK!");
                });
            }

        first_open = false;
    });
}
