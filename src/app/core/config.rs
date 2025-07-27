use std::fs::File;
use std::{
    fs,
    io::{self, BufRead, Write},
    path::PathBuf,
};

pub fn redshift_cfg_path() -> PathBuf {
    glib::user_config_dir().join("redshift/config")
}

pub fn load_current_temp() -> Option<u32> {
    let path = redshift_cfg_path();
    let file = File::open(&path).ok()?;
    for line in io::BufReader::new(file).lines().map_while(Result::ok) {
        if let Some(v) = line.strip_prefix("temp=") {
            if let Ok(k) = v.trim().parse::<u32>() {
                return Some(k);
            }
        }
    }
    None
}

// atomic write
pub fn write_temp_atomic(val: u32) -> io::Result<()> {
    let path = redshift_cfg_path();
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }

    let mut lines = Vec::<String>::new();
    let mut found = false;
    if let Ok(file) = File::open(&path) {
        // ⬇ вместо .flatten()
        for line in io::BufReader::new(file).lines().map_while(Result::ok) {
            if line.starts_with("temp=") {
                lines.push(format!("temp={val}"));
                found = true;
            } else {
                lines.push(line);
            }
        }
    }
    if !found {
        lines.push(format!("temp={val}"));
    }

    let tmp = path.with_extension("tmp");
    {
        let mut f = File::create(&tmp)?;
        for l in &lines {
            writeln!(f, "{l}")?;
        }
        f.flush()?;
    }
    fs::rename(&tmp, &path)?;
    Ok(())
}
