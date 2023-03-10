
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

mod application;
mod server_interface;
mod message;
mod user_state;

use crate::application::Application;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 300.0)),
        ..Default::default()
    };

    eframe::run_native(
        "ChadGPT",
        options,
        Box::new(|_cc| Box::new(Application::default()))
    ).expect("Application Crashed");
}
