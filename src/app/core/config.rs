use super::paths::config_dir;

use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, Write},
    path::PathBuf,
};

pub fn redshift_cfg_path() -> PathBuf {
    config_dir().join("redshift/config")
}

pub fn load_current_temp() -> Option<u32> {
    let file = File::open(redshift_cfg_path()).ok()?;

    for line in BufReader::new(file).lines().map_while(Result::ok) {
        if let Some(value) = line.strip_prefix("temp=")
            && let Ok(value) = value.trim().parse::<u32>()
        {
            return Some(value);
        }
    }

    None
}

pub fn write_temp_atomic(val: u32) -> io::Result<()> {
    let path = redshift_cfg_path();

    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }

    let mut lines = Vec::new();
    let mut found = false;

    match File::open(&path) {
        Ok(file) => {
            for line in BufReader::new(file).lines() {
                let line = line?;
                if line.starts_with("temp=") {
                    lines.push(format!("temp={val}"));
                    found = true;
                } else {
                    lines.push(line);
                }
            }
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }

    if !found {
        lines.push(format!("temp={val}"));
    }

    let tmp = path.with_extension("tmp");
    {
        let mut file = File::create(&tmp)?;
        for line in &lines {
            writeln!(file, "{line}")?;
        }
        file.flush()?;
    }

    fs::rename(tmp, path)
}
