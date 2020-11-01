use crate::state::State;
use imgui::*;
use session::TypeCode;
use std::sync::{Arc, RwLock, RwLockReadGuard};

pub struct Drawer<'a> {
    nodes: Vec<Node<'a>>,
    name: String,
    opened: (bool, Condition),
}

impl<'a> Drawer<'a> {
    pub fn new(name: String) -> Self {
        Self {
            nodes: vec![],
            name,
            opened: (false, Condition::FirstUseEver),
        }
    }

    pub fn opened(mut self, opened: bool, cond: Condition) -> Self {
        self.opened = (opened, cond);
        self
    }

    pub fn with_node(mut self, node: Node<'a>) -> Self {
        self.nodes.push(node);
        self
    }

    pub fn build<F: Fn()>(self, state: &RwLockReadGuard<State>, ui: &Ui, f: F) {
        TreeNode::new(unsafe { ImStr::from_utf8_with_nul_unchecked(format!("{}\0", self.name).as_bytes()) })
            .opened(self.opened.0, self.opened.1)
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
                Drawer::new("Audio".to_owned())
                    .with_node(Node {
                        node: TreeNode::new(im_str!("AcousticSpaceNode")).label(im_str!("Acoustic Space")),
                        code: "ASPC",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("MusicTrackNode")).label(im_str!("Music Track")),
                        code: "MUST",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("MusicTypeNode")).label(im_str!("Music Type")),
                        code: "MUSC",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("ReverbNode")).label(im_str!("Reverb Parameters")),
                        code: "REVB",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("SoundCategoryNode")).label(im_str!("Sound Category")),
                        code: "SNCT",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("SoundDescriptorNode")).label(im_str!("Sound Descriptor")),
                        code: "SNDR",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("SoundMarkerNode")).label(im_str!("Sound Marker")),
                        code: "SOUN",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("SoundOutputModelNode")).label(im_str!("Sound Output Model")),
                        code: "SOPM",
                    }),
            )
            .with_drawer(
                Drawer::new("Character".to_owned())
                    .with_node(Node {
                        node: TreeNode::new(im_str!("AIPackageNode")).label(im_str!("AI Package")),
                        code: "PACK",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("AssociationTypeNode")).label(im_str!("Association Type")),
                        code: "ASTP",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("ClassNode")).label(im_str!("Class")),
                        code: "CLAS",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("EquipSlotNode")).label(im_str!("Equip Slot")),
                        code: "EQUP",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("FactionNode")).label(im_str!("Faction")),
                        code: "FACT",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("HeadPartNode")).label(im_str!("HeadPart")),
                        code: "HDPT",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("MovementTypeNode")).label(im_str!("Movement Type")),
                        code: "MOVT",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("QuestNode")).label(im_str!("Quest")),
                        code: "QUST",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("RaceNode")).label(im_str!("Race")),
                        code: "RACE",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("RelationshipNode")).label(im_str!("Relationship")),
                        code: "RELA",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("StoryManagerEventNode")).label(im_str!("Story Manager Event Node")),
                        code: "SMEN",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("VoiceTypeNode")).label(im_str!("Voice Type")),
                        code: "VTYP",
                    }),
            )
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
                        node: TreeNode::new(im_str!("ArmorAddonNode")).label(im_str!("Armor Addon")),
                        code: "ARMA",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("BookNode")).label(im_str!("Book")),
                        code: "BOOK",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("ConstructibleObjectNode")).label(im_str!("Constructible Object")),
                        code: "COBJ",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("IngredientNode")).label(im_str!("Ingredient")),
                        code: "INGR",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("KeyNode")).label(im_str!("Key")),
                        code: "KEYM",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("LeveledItemNode")).label(im_str!("Leveled Item")),
                        code: "LVLI",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("MiscItemNode")).label(im_str!("Misc. Item")),
                        code: "MISC",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("OutfitNode")).label(im_str!("Outfit")),
                        code: "OTFT",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("SoulGemNode")).label(im_str!("Soul Gem")),
                        code: "SLGM",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("ArmorNode")).label(im_str!("Armor")),
                        code: "ARMO",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("WeaponNode")).label(im_str!("Weapon")),
                        code: "WEAP",
                    }),
            )
            .with_drawer(Drawer::new("Magic".to_owned()))
            .with_drawer(
                Drawer::new("Miscellaneous".to_owned())
                    .with_node(Node {
                        node: TreeNode::new(im_str!("AnimationObjectNode")).label(im_str!("Animation Object")),
                        code: "ANIO",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("ArtObjectNode")).label(im_str!("Art Object")),
                        code: "ARTO",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("CollisionLayerNode")).label(im_str!("Collision Layer")),
                        code: "COLL",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("ColorFormNode")).label(im_str!("Color Form")),
                        code: "CLFM",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("CombatStyleNode")).label(im_str!("Combat Style")),
                        code: "CSTY",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("FormListNode")).label(im_str!("Form List")),
                        code: "FLST",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("GlobalNode")).label(im_str!("Global")),
                        code: "GLOB",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("IdleMarkerNode")).label(im_str!("Idle Marker")),
                        code: "IDLM",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("KeywordNode")).label(im_str!("Keyword")),
                        code: "KYWD",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("LandTextureNode")).label(im_str!("Land Texture")),
                        code: "LTEX",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("LoadScreenNode")).label(im_str!("Load Screen")),
                        code: "LSCR",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("MaterialObjectNode")).label(im_str!("Material Object")),
                        code: "MATO",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("MessageNode")).label(im_str!("Message")),
                        code: "MESG",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("TextureSetNode")).label(im_str!("Texture Set")),
                        code: "TXST",
                    }),
            )
            .with_drawer(Drawer::new("Special Effects".to_owned()))
            .with_drawer(Drawer::new("World Data".to_owned()))
            .with_drawer(
                Drawer::new("World Objects".to_owned())
                    .with_node(Node {
                        node: TreeNode::new(im_str!("ActivatorNode")).label(im_str!("Activator")),
                        code: "ACTI",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("ContainerNode")).label(im_str!("Container")),
                        code: "CONT",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("DoorNode")).label(im_str!("Door")),
                        code: "DOOR",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("FloraNode")).label(im_str!("Flora")),
                        code: "FLOR",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("FurnitureNode")).label(im_str!("Furniture")),
                        code: "FURN",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("GrassNode")).label(im_str!("Grass")),
                        code: "GRAS",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("LightNode")).label(im_str!("Light")),
                        code: "LIGH",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("MovableStaticNode")).label(im_str!("Movable Static")),
                        code: "MSTT",
                    })
                    .with_node(Node {
                        node: TreeNode::new(im_str!("StaticNode")).label(im_str!("Static")),
                        code: "STAT",
                    })
                    // TERMINAL?
                    .with_node(Node {
                        node: TreeNode::new(im_str!("TreeNode")).label(im_str!("Tree")),
                        code: "TREE",
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
    let records = state.plugin.get_records_by_code(code).unwrap();

    records.iter().enumerate().for_each(|(pos, r)| {
        let id_string = format!("{}Leaf{}{}", code.to_utf8().unwrap(), pos, "\0");
        let form_id = format!("({:#010x})", r.header.id);
        let label = format!("{} {}\0", form_id, r.editor_id().unwrap_or("Unnamed".to_owned()));

        TreeNode::new(unsafe { ImStr::from_utf8_with_nul_unchecked(id_string.as_bytes()) })
            .leaf(true)
            .label(unsafe { ImStr::from_utf8_with_nul_unchecked(label.as_bytes()) })
            .build(&ui, || {
                f();
            });
    });
}
