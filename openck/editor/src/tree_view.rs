use crate::state::State;
use imgui::*;
use session::TypeCode;
use std::sync::{Arc, RwLock, RwLockReadGuard};

pub struct Drawer<'a> {
    nodes: Vec<Node<'a>>,
    name: String,
}

impl<'a> Drawer<'a> {
    pub fn new(name: String) -> Self {
        Self { nodes: vec![], name }
    }

    pub fn with_node(mut self, node: Node<'a>) -> Self {
        self.nodes.push(node);
        self
    }

    pub fn build<F: Fn()>(self, state: &RwLockReadGuard<State>, ui: &Ui, f: F) {
        TreeNode::new(unsafe { ImStr::from_utf8_with_nul_unchecked(format!("{}\0", self.name).as_bytes()) })
            .opened(true, Condition::FirstUseEver)
            .build(ui, || {
                self.nodes.iter().for_each(|n| n.build(state, ui, || f()));
            });
    }
}

pub struct Node<'a> {
    node: TreeNode<'a>,
    code: &'a str,
}

impl<'a> Node<'a> {
    pub fn build<F: Fn()>(&self, state: &RwLockReadGuard<State>, ui: &Ui, f: F) {
        self.node.build(ui, || {
            build_nodes_from_type(TypeCode::from_utf8(self.code).unwrap(), &state, &ui, || f());
        });
    }
}

pub struct TreeView<'a> {
    drawers: Vec<Drawer<'a>>,
    nodes: Vec<Node<'a>>,
}

impl<'a> TreeView<'a> {
    pub fn new() -> Self {
        Self {
            drawers: vec![],
            nodes: vec![],
        }
    }

    pub fn with_drawer(mut self, drawer: Drawer<'a>) -> Self {
        self.drawers.push(drawer);
        self
    }

    pub fn with_node(mut self, node: Node<'a>) -> Self {
        self.nodes.push(node);
        self
    }

    pub fn display(self, ui: &Ui, ui_state: Arc<RwLock<State>>) {
        let tree_view = self
            .with_node(Node {
                node: TreeNode::new(im_str!("ActorNode")).label(im_str!("Actors")),
                code: "NPC_",
            })
            .with_drawer(
                Drawer::new("Items".to_owned())
                    .with_node(Node {
                        node: TreeNode::new(im_str!("AmmoNode")).label(im_str!("Ammo")),
                        code: "AMMO",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("ArmorNode")).label(im_str!("Armor")),
                        code: "ARMO",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("BookNode")).label(im_str!("Book")),
                        code: "BOOK",
                    }),
            );

        tree_view.build(ui, &ui_state);
    }

    pub fn build(self, ui: &Ui, ui_state: &Arc<RwLock<State>>) {
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

                for node in self.nodes {
                    node.build(&state, ui, || {});
                }

                for drawer in self.drawers {
                    drawer.build(&state, ui, || {});
                }
            });
    }
}

fn build_nodes_from_type<F: Fn()>(code: TypeCode, state: &RwLockReadGuard<State>, ui: &Ui, f: F) {
    let records = state.plugin.get_records_by_code(TypeCode::from_utf8("NPC_").unwrap()).unwrap();

    records.iter().enumerate().for_each(|(pos, r)| {
        let id_string = format!("{}Leaf{}{}", code.to_utf8().unwrap(), pos, "\0");
        let label = format!("{}\0", r.editor_id().unwrap_or("Unnamed".to_owned()));

        TreeNode::new(unsafe { ImStr::from_utf8_with_nul_unchecked(id_string.as_bytes()) })
            .leaf(true)
            .label(unsafe { ImStr::from_utf8_with_nul_unchecked(label.as_bytes()) })
            .build(&ui, || {
                f();
            });
    });
}
