#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use directories::{BaseDirs, ProjectDirs, UserDirs};
use std::{io, path::PathBuf};

fn config() -> eframe::NativeOptions {
    // Some logging and the likes
    {
        // Silence wgpu log spam (https://github.com/gfx-rs/wgpu/issues/3206)
        let mut rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_owned());
        for loud_crate in ["naga", "wgpu_core", "wgpu_hal"] {
            if !rust_log.contains(&format!("{loud_crate}=")) {
                rust_log += &format!(",{loud_crate}=warn");
            }
        }
        std::env::set_var("RUST_LOG", rust_log);
    }

    eframe::NativeOptions {
        #[cfg(feature = "wgpu")]
        renderer: eframe::Renderer::Wgpu,

        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        ..Default::default()
    }
}

fn main() {
    let options = config();

    env_logger::builder()
     .filter_level(log::LevelFilter::Debug).init();
   

    eframe::run_native("Money Man", options, Box::new(|cc| {
        Ok(gui::start(cc))
    }));

}
