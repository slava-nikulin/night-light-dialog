pub mod core;
pub mod tray;

use ksni::blocking::TrayMethods;

use core::presets::load_presets;
use tray::NightLightTray;

pub fn run() {
    let tray = NightLightTray::new(load_presets());
    let _handle = tray.spawn().expect("failed to start system tray service");

    loop {
        std::thread::park();
    }
}
