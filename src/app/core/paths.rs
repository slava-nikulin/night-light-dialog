use std::{env, path::PathBuf};

pub fn config_dir() -> PathBuf {
    env::var_os("XDG_CONFIG_HOME")
        .filter(|path| !path.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            let home = env::var_os("HOME").expect("neither XDG_CONFIG_HOME nor HOME is set");
            PathBuf::from(home).join(".config")
        })
}
