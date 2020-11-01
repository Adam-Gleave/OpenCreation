use crate::state::State;
use imgui::*;
use session::TypeCode;
use std::sync::{Arc, RwLock};

pub struct TreeView {}

impl TreeView {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(self, ui: &Ui, ui_state: Arc<RwLock<State>>) {
        Window::new(im_str!("Object View"))
            .position([10f32, 10f32 + ui_state.read().unwrap().menu_y], Condition::FirstUseEver)
            .size(
                [
                    400f32,
                    ui_state.read().unwrap().screen_y as f32 - (20f32 + ui_state.read().unwrap().menu_y),
                ],
                Condition::FirstUseEver,
            )
            .build(&ui, || {
                let lock = Arc::clone(&ui_state);
                let state = lock.read().unwrap();

                let actors = state.plugin.get_records_by_code(TypeCode::from_utf8("NPC_").unwrap()).unwrap();

                TreeNode::new(im_str!("TreeNode"))
                    .label(im_str!("Plugin File"))
                    .opened(true, Condition::FirstUseEver)
                    .build(ui, || {
                        TreeNode::new(im_str!("ActorNode"))
                            .label(im_str!("Actors"))
                            .opened(true, Condition::FirstUseEver)
                            .build(ui, || {
                                actors.iter().enumerate().for_each(|(pos, r)| {
                                    let id_string = format!("ActorLeaf{}{}", pos, "\0");
                                    let text_string = format!("ID: {:#010x}{}", r.header.id, "\0");
                                    TreeNode::new(unsafe { ImStr::from_utf8_with_nul_unchecked(id_string.as_bytes()) })
                                        .leaf(true)
                                        .label(unsafe { ImStr::from_utf8_with_nul_unchecked(text_string.as_bytes()) })
                                        .build(ui, || {});
                                });
                            });
                    });
            });
    }
}
