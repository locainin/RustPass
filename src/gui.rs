extern crate gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, CheckButton, Entry, TextView, Label};
use crate::generator::PasswordGenerator;
use std::collections::HashSet;
use crate::utils::CharClass;

pub fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title("Password Generator");
    window.set_default_size(350, 200);

    let vbox = Box::new(gtk::Orientation::Vertical, 5);

    let length_entry = Entry::new();
    length_entry.set_placeholder_text(Some("Enter password length"));

    let exclude_entry = Entry::new();
    exclude_entry.set_placeholder_text(Some("Enter characters to exclude"));

    let upper_case_checkbox = CheckButton::with_label("Include Uppercase Letters");
    let lower_case_checkbox = CheckButton::with_label("Include Lowercase Letters");
    let numbers_checkbox = CheckButton::with_label("Include Numbers");
    let special_chars_checkbox = CheckButton::with_label("Include Special Characters");

    upper_case_checkbox.set_active(true);
    lower_case_checkbox.set_active(true);
    numbers_checkbox.set_active(true);
    special_chars_checkbox.set_active(true);

    let generate_button = Button::with_label("Generate Password");

    let result_textview = TextView::new();
    result_textview.set_editable(false);
    result_textview.set_cursor_visible(false);

    let error_label = Label::new(None);
    error_label.set_halign(gtk::Align::Start);

    vbox.pack_start(&length_entry, false, false, 0);
    vbox.pack_start(&exclude_entry, false, false, 0);
    vbox.pack_start(&upper_case_checkbox, false, false, 0);
    vbox.pack_start(&lower_case_checkbox, false, false, 0);
    vbox.pack_start(&numbers_checkbox, false, false, 0);
    vbox.pack_start(&special_chars_checkbox, false, false, 0);
    vbox.pack_start(&generate_button, false, false, 0);
    vbox.pack_start(&result_textview, false, false, 0);
    vbox.pack_start(&error_label, false, false, 0);

    window.add(&vbox);

    generate_button.connect_clicked(move |_| {
        error_label.set_text(""); // Clear previous error message

        let length = match length_entry.get_text().parse::<usize>() {
            Ok(l) => l,
            Err(_) => {
                error_label.set_text("Invalid length. Please enter a valid number.");
                return;
            }
        };

        let excluded_chars = exclude_entry.get_text().to_string();
        
        let mut generator = PasswordGenerator::new();
        generator.set_length(length);
        generator.set_excluded_character_set(excluded_chars);

        let mut char_classes: HashSet<CharClass> = HashSet::new();
        if upper_case_checkbox.get_active() {
            char_classes.insert(CharClass::UpperLetters);
        }
        if lower_case_checkbox.get_active() {
            char_classes.insert(CharClass::LowerLetters);
        }
        if numbers_checkbox.get_active() {
            char_classes.insert(CharClass::Numbers);
        }
        if special_chars_checkbox.get_active() {
            char_classes.insert(CharClass::SpecialCharacters);
        }

        if char_classes.is_empty() {
            error_label.set_text("Please select at least one character class.");
            return;
        }

        generator.set_char_classes(char_classes);

        let password = generator.generate_password();
        result_textview.get_buffer().unwrap().set_text(&password);
    });

    window.show_all();
}




