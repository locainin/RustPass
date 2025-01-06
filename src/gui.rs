use gtk4::prelude::*;
use gtk4::{
    gdk, Application, ApplicationWindow, Box as GtkBox, Button, CheckButton, CssProvider, Entry,
    HeaderBar, Image, Label, Orientation, ProgressBar, Separator, TextView, WrapMode,
};
use std::collections::HashSet;

// Your modules (adjust paths as needed):
use crate::calculate_password_strength::{calculate_entropy, get_strength};
use crate::generator::PasswordGenerator;
use crate::utils::CharClass;

pub fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Password Generator")
        .default_width(400)
        .default_height(400)
        .build();

    // Name for CSS targeting
    window.set_widget_name("password_generator_window");

    // Header bar + title container
    let header = HeaderBar::builder()
        .decoration_layout("close")
        .build();
    let title_box = GtkBox::new(Orientation::Vertical, 0);

    let title_label = Label::builder()
        .label("Password Generator")
        .css_classes(vec!["title-label"])
        .build();
    let subtitle_label = Label::builder()
        .label("Create secure passwords easily")
        .css_classes(vec!["subtitle-label"])
        .build();

    title_box.append(&title_label);
    title_box.append(&subtitle_label);

    header.set_title_widget(Some(&title_box));
    window.set_titlebar(Some(&header));

    // Main container
    let vbox = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_start(16)
        .margin_end(16)
        .margin_top(16)
        .margin_bottom(16)
        .build();

    // Password length entry
    let length_entry = Entry::builder()
        .placeholder_text("Enter password length")
        .build();

    // Characters to exclude entry
    let exclude_entry = Entry::builder()
        .placeholder_text("Enter characters to exclude")
        .build();

    // Checkboxes
    let upper_case_checkbox = CheckButton::builder()
        .label("Include Uppercase Letters")
        .build();
    let lower_case_checkbox = CheckButton::builder()
        .label("Include Lowercase Letters")
        .build();
    let numbers_checkbox = CheckButton::builder().label("Include Numbers").build();
    let special_chars_checkbox = CheckButton::builder()
        .label("Include Special Characters")
        .build();

    upper_case_checkbox.set_active(true);
    lower_case_checkbox.set_active(true);
    numbers_checkbox.set_active(true);
    special_chars_checkbox.set_active(true);

    // Generate button (just text initially)
    let generate_button = Button::builder()
        .label("Generate Password")
        .tooltip_text("Click to generate a secure password")
        .build();
    generate_button.style_context().add_class("generate-button");

    // TextView for the generated password
    let result_textview = TextView::builder()
        .editable(false)
        .cursor_visible(false)
        .wrap_mode(WrapMode::Word)
        .build();

    // Copy button, initially hidden
    let copy_button = Button::builder()
        .label("Copy Password")
        .tooltip_text("Copy the generated password to clipboard")
        .visible(false)
        .build();
    copy_button.style_context().add_class("copy-button");

    // Error label
    let error_label = Label::builder()
        .css_classes(vec!["error"])
        .visible(false)
        .build();

    // Progress bar + strength label
    let strength_bar = ProgressBar::builder().show_text(true).build();
    let strength_label = Label::builder()
        .css_classes(vec!["strength-label"])
        .build();

    // Layout
    vbox.append(&length_entry);
    vbox.append(&exclude_entry);
    vbox.append(&upper_case_checkbox);
    vbox.append(&lower_case_checkbox);
    vbox.append(&numbers_checkbox);
    vbox.append(&special_chars_checkbox);

    vbox.append(&Separator::new(Orientation::Horizontal));
    vbox.append(&generate_button);
    vbox.append(&error_label);
    vbox.append(&strength_bar);
    vbox.append(&strength_label);
    vbox.append(&result_textview);
    vbox.append(&copy_button);

    // Load the CSS
    let css = match std::fs::read_to_string("src/style.css") {
        Ok(data) => data,
        Err(_) => {
            eprintln!("Failed to read CSS file. Falling back to default style.");
            return;
        }
    };
    let provider = CssProvider::new();
    provider.load_from_data(&css);

    gtk4::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Error initializing display"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    window.set_child(Some(&vbox));

    // Generate button logic
    generate_button.connect_clicked({
        let length_entry = length_entry.clone();
        let exclude_entry = exclude_entry.clone();
        let result_textview = result_textview.clone();
        let error_label = error_label.clone();
        let strength_bar = strength_bar.clone();
        let strength_label = strength_label.clone();
        let upper_case_checkbox = upper_case_checkbox.clone();
        let lower_case_checkbox = lower_case_checkbox.clone();
        let numbers_checkbox = numbers_checkbox.clone();
        let special_chars_checkbox = special_chars_checkbox.clone();
        let copy_button = copy_button.clone();
        let generate_button = generate_button.clone();

        move |_| {
            error_label.set_text("");
            error_label.set_visible(false);

            // Parse length
            let length = match length_entry.text().parse::<usize>() {
                Ok(l) => l,
                Err(_) => {
                    error_label.set_visible(true);
                    error_label.set_text("Invalid length. Please enter a valid number.");
                    return;
                }
            };

            let excluded_chars = exclude_entry.text().to_string();
            let mut generator = PasswordGenerator::new();
            generator.set_length(length);
            generator.set_excluded_character_set(excluded_chars);

            // Collect character classes
            let mut char_classes: HashSet<CharClass> = HashSet::new();
            if upper_case_checkbox.is_active() {
                char_classes.insert(CharClass::UpperLetters);
            }
            if lower_case_checkbox.is_active() {
                char_classes.insert(CharClass::LowerLetters);
            }
            if numbers_checkbox.is_active() {
                char_classes.insert(CharClass::Numbers);
            }
            if special_chars_checkbox.is_active() {
                char_classes.insert(CharClass::SpecialCharacters);
            }

            if char_classes.is_empty() {
                error_label.set_visible(true);
                error_label.set_text("Please select at least one character class.");
                return;
            }

            generator.set_char_classes(char_classes);

            // Generate and display password
            let password = generator.generate_password();
            let buffer = result_textview.buffer();
            buffer.set_text(&password);

            // Calculate & display strength
            let entropy = calculate_entropy(&password);
            let strength_score = (entropy / 60.0).clamp(0.0, 1.0);
            strength_bar.set_fraction(strength_score);
            let strength_text = get_strength(entropy);
            strength_bar.set_text(Some(&strength_text));
            strength_label.set_text(&strength_text);

            // Now that a password exists, show the Copy button
            copy_button.set_visible(true);

            // Switch from "Generate Password" text to "Regenerate Password" with a refresh icon
            generate_button.set_label("Regenerate Password");
            let refresh_icon = Image::from_icon_name("view-refresh");
            refresh_icon.set_pixel_size(24);
            generate_button.set_child(Some(&refresh_icon));
        }
    });

    // Copy button logic
    copy_button.connect_clicked({
        let result_textview = result_textview.clone();
        move |_| {
            let buffer = result_textview.buffer();
            let password = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false);

            if let Some(display) = gdk::Display::default() {
                let clipboard = display.clipboard();
                clipboard.set_text(&password);
            }
        }
    });

    window.show();
}
