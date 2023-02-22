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
        "My Application",
        options,
        Box::new(|_cc| Box::new(Application::default()))
    ).expect("Application Crashed");
}
