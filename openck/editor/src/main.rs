mod imgui_util;

use imgui::*;

fn main() {
    let mut system = imgui_util::init(file!());

    system.imgui.style_mut().window_rounding = 0f32;
    system.imgui.io_mut().config_flags |= imgui::ConfigFlags::DOCKING_ENABLE;
    system.imgui.io_mut().docking_with_shift = true;

    let (x, y) = system.display.get_framebuffer_dimensions();
    println!("{}, {}", x, y);

    let mut first_open = true;
    let mut about_open = true;

    system.main_loop(move |_, ui| {
        Window::new(im_str!("Scene"))
            .flags(imgui::WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS)
            .build(ui, || {});
        Window::new(im_str!("Tree View"))
            .flags(imgui::WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS)
            .build(ui, || {});
        Window::new(im_str!("Scene Objects"))
            .flags(imgui::WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS)
            .build(ui, || {});

        if let Some(menu_bar) = ui.begin_main_menu_bar() {
            if let Some(menu) = ui.begin_menu(im_str!("File"), true) {
                MenuItem::new(im_str!("Data..."))
                    .enabled(true)
                    .build(ui);

                menu.end(ui);
            }
            menu_bar.end(ui);
        }

        if first_open {
            imgui::Dock::new().build(|root| {
                root.size([1280f32, 800f32]).position([0f32, 18f32]).split(
                    imgui::Direction::Left, 
                    0.3f32,
                    |left| {
                        left.dock_window(im_str!("Tree View"));
                    }, |right| {
                        right.split(
                            imgui::Direction::Up, 
                            0.7f32,
                            |up| {
                                up.dock_window(im_str!("Scene"));
                            }, |down| {
                                down.dock_window(im_str!("Scene Objects"));
                            })
                    })
            });
        }

        if about_open {
            Window::new(im_str!("About"))
                .size([300f32, 100f32], Condition::Always)
                .opened(&mut about_open)
                .build(ui, || {
                    ui.text("OpenCK v0.0.1");
                    ui.text("Welcome to the OpenCK!");
                });
            }

        first_open = false;
    });
}
