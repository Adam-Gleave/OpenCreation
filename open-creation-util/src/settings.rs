use std::{default::Default, fs::File, io::Read, path::Path};

use toml::Value as Toml;

static SETTINGS_FILENAME: &'static str = "../config.toml";

#[derive(Debug)]
pub struct Settings {
    data_path: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self { data_path: "/Data/".to_string() }
    }
}

impl Settings {
    pub fn load() -> Settings {
        let mut input = String::new();
        File::open(Path::new(SETTINGS_FILENAME)).and_then(|mut f| f.read_to_string(&mut input)).unwrap();

        let mut settings = Settings::default();

        if let Ok(toml) = input.parse() {
            if let Toml::Table(toml) = toml {
                if let Some(paths) = toml.get("paths") {
                    if let Some(Toml::String(data_path)) = paths.get("data_path") {
                        settings.data_path = data_path.to_string();
                    }
                }
            }
        }

        settings
    }
}

mod tests {
    use super::Settings;

    #[test]
    fn load() {
        let settings = Settings::load();
        assert_eq!(settings.data_path.as_str(), "/Data/");
    }

    #[test]
    fn default() {
        let settings = Settings::default();
        assert_eq!(settings.data_path.as_str(), "/Data/");
    }
}