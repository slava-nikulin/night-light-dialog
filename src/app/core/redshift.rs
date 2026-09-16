use super::config::{load_current_temp, write_temp_atomic};

use std::{io, process::Command};

pub fn apply_temperature(val: u32) -> io::Result<()> {
    if load_current_temp() == Some(val) {
        return Ok(());
    }

    run_redshift(&["-x"])?;

    let value = val.to_string();
    run_redshift(&["-O", &value])?;

    write_temp_atomic(val)?;

    Ok(())
}

fn run_redshift(args: &[&str]) -> io::Result<()> {
    let output = Command::new("redshift").args(args).output()?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stderr = stderr.trim();

    let message = if stderr.is_empty() {
        format!("redshift {args:?} exited with {}", output.status)
    } else {
        format!("redshift {args:?} failed: {stderr}")
    };

    Err(io::Error::other(message))
}
