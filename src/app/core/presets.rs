use super::paths::config_dir;

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
    config_dir().join("night-light/presets.toml")
}

pub fn load_presets() -> Vec<Preset> {
    fs::read_to_string(presets_path())
        .ok()
        .and_then(|contents| toml::from_str::<PresetsFile>(&contents).ok())
        .map(|file| file.presets)
        .filter(|presets| !presets.is_empty())
        .unwrap_or_else(default_presets)
}

fn default_presets() -> Vec<Preset> {
    [
        ("1K", 1000),
        ("1.5K", 1500),
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
    .map(|(label, value)| Preset {
        label: label.into(),
        value,
    })
    .collect()
}
