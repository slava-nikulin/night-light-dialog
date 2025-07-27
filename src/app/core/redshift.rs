use super::config::{load_current_temp, write_temp_atomic};
use std::process::Command;
use which::which;

pub fn apply_temperature(val: u32) {
    if load_current_temp() == Some(val) {
        return;
    }
    if let Err(e) = write_temp_atomic(val) {
        eprintln!("write_temp failed: {e}");
        return;
    }
    if which("redshift").is_ok() {
        let _ = Command::new("redshift").arg("-x").status();
        let _ = Command::new("redshift")
            .args(["-O", &val.to_string()])
            .status();
    }
}
