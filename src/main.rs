use gtk4::prelude::*;
use gtk4::Application;

mod generator;
mod utils;
mod gui;
mod calculate_password_strength;

fn main() {
    let application = Application::builder()
        .application_id("passwordgenerator")
        .build();

    application.connect_activate(|app| {
        gui::build_ui(app);
    });

    application.run();
}
