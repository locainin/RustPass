extern crate gtk;
extern crate rand;

mod generator;
mod utils;
mod gui;
mod calculate_password_strength;



use gtk::prelude::ApplicationExt;
use gtk::prelude::ApplicationExtManual;
use gtk::Application;

fn main() {
    let application = Application::new(
        Some("com.example.passwordgenerator"),
        Default::default(),
    );

    application.connect_activate(|app| {
        gui::build_ui(app);
    });

    application.run();
}