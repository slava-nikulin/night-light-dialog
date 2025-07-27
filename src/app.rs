pub mod core;
pub mod ui;

use core::presets::load_presets;
use gtk4::Application;
use gtk4::prelude::*;
use ui::window::WindowBuilder;

pub fn run() {
    let app = Application::builder()
        .application_id("org.example.NightLightDiagGTK4")
        .build();

    app.connect_activate(|app| {
        let presets = load_presets();
        WindowBuilder { app, presets }.build();
    });

    app.run();
}
