use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Deserialize)]
pub struct Preset {
    pub label: String,
    pub value: u32,
}

#[derive(Debug, Deserialize)]
struct PresetsFile {
    presets: Vec<Preset>,
}

pub fn presets_path() -> PathBuf {
    // Updated to match actual config location
    glib::user_config_dir().join("nighlight/presets.toml")
}

pub fn load_presets() -> Vec<Preset> {
    let p = presets_path();
    if let Ok(s) = fs::read_to_string(&p) {
        if let Ok(parsed) = toml::from_str::<PresetsFile>(&s) {
            if !parsed.presets.is_empty() {
                return parsed.presets;
            }
        }
    }
    // defaults
    [
        ("2K", 2000),
        ("2.5K", 2500),
        ("3K", 3000),
        ("3.5K", 3500),
        ("4K", 4000),
        ("4.5K", 4500),
        ("5K", 5000),
        ("5.5K", 5500),
        ("6K", 6000),
        ("6.5K", 6500),
    ]
    .into_iter()
    .map(|(l, v)| Preset {
        label: l.into(),
        value: v,
    })
    .collect()
}
