extern crate gtk;
extern crate rand;

mod generator;
mod utils;
mod gui;

use gtk::prelude::*;
use gtk::Application;
use gio::prelude::*; // Import traits from gio

fn main() {
    let application = Application::new(
        Some("com.example.passwordgenerator"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        gui::build_ui(app);
    });

    application.run(&[]);
}

