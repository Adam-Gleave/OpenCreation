#[rustfmt::skip]
pub use tes_parser::{
    ParseError,
    Record,
    TypeCode,
    read_plugin
};

use std::collections::HashMap;
use std::fs::File;
pub struct Plugin {
    plugin_data: tes_parser::Plugin,
    record_map: HashMap<u32, RecordIndex>,
    base: bool,
}

struct RecordIndex {
    pub group_pos: usize,
    pub record_pos: usize,
}

impl Plugin {
    pub fn from_file(f: File, is_base: bool) -> Result<Self, ParseError> {
        let plugin_data = read_plugin(f)?;

        Ok(Self {
            record_map: generate_maps(&plugin_data),
            plugin_data,
            base: is_base,
        })
    }

    pub fn get_records_by_code(&self, code: TypeCode) -> Option<&Vec<Record>> {
        let group = self.plugin_data.top_groups.iter().find(|g| g.top_group_matches_type(code));

        if let Some(group) = group {
            Some(&group.records)
        } else {
            None
        }
    }

    pub fn get_records_by_code_mut(&mut self, code: TypeCode) -> Option<&mut Vec<Record>> {
        let group = self.plugin_data.top_groups.iter_mut().find(|g| g.top_group_matches_type(code));

        if let Some(group) = group {
            Some(&mut group.records)
        } else {
            None
        }
    }

    pub fn get_record(&self, id: u32) -> Option<&Record> {
        if let Some(index) = self.record_map.get(&id) {
            Some(&self.plugin_data.top_groups[index.group_pos].records[index.record_pos])
        } else {
            None
        }
    }

    pub fn get_record_mut(&mut self, id: u32) -> Option<&mut Record> {
        if let Some(index) = self.record_map.get(&id) {
            Some(&mut self.plugin_data.top_groups[index.group_pos].records[index.record_pos])
        } else {
            None
        }
    }
}

fn generate_maps(plugin_data: &tes_parser::Plugin) -> HashMap<u32, RecordIndex> {
    let groups = &plugin_data.top_groups;
    let mut map = HashMap::new();

    for (group_pos, group) in groups.iter().enumerate() {
        for (record_pos, record) in group.records.iter().enumerate() {
            map.insert(record.header.id, RecordIndex { record_pos, group_pos });
        }
    }
    map
}
