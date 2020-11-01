use session::{Plugin, Record, TypeCode};
use std::fs::File;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

pub struct State {
    pub screen_x: f32,
    pub screen_y: f32,
    pub menu_y: f32,

    pub plugin: Plugin,
}

impl State {
    pub fn new() -> Self {
        let path = PathBuf::from(format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/../../data/Skyrim.esm"));
        println!("Path: {:#?}", path);
        let file = File::open(path).unwrap();
        let plugin = Plugin::from_file(file, true).unwrap();

        Self {
            screen_x: 0f32,
            screen_y: 0f32,
            menu_y: 18f32,
            plugin,
        }
    }
}
