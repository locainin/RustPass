extern crate gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, CheckButton, Entry, TextView, Label, HeaderBar, Separator, CssProvider, StyleContext, STYLE_PROVIDER_PRIORITY_USER, Image, ProgressBar};
use crate::generator::PasswordGenerator;
use std::collections::HashSet;
use crate::utils::CharClass;
use crate::calculate_password_strength::{calculate_entropy, get_strength};

pub fn build_ui(app: &Application) {
    // Create the main application window
    let window = ApplicationWindow::new(app);
    window.set_title("Password Generator");
    window.set_default_size(400, 400); // Set the default size of the application window
    window.set_border_width(10); // Set border width for padding

    // Add a modern header bar to the window
    let header = HeaderBar::new();
    header.set_title(Some("Password Generator")); // Set the title of the header bar
    header.set_subtitle(Some("Create secure passwords easily")); // Set the subtitle for additional context
    header.set_show_close_button(true); // Show the close button in the header bar
    window.set_titlebar(Some(&header)); // Set the header bar as the title bar for the window

    // Create a vertical box container to hold all the widgets, with spacing of 10 pixels
    let vbox = Box::new(gtk::Orientation::Vertical, 10);

    // Add a separator for a cleaner look between sections
    let separator = Separator::new(gtk::Orientation::Horizontal);

    // Entry field for the password length
    let length_entry = Entry::new();
    length_entry.set_placeholder_text(Some("Enter password length")); // Placeholder text for guidance
    length_entry.set_margin_top(5);
    length_entry.set_margin_bottom(5);

    // Entry field for characters to exclude
    let exclude_entry = Entry::new();
    exclude_entry.set_placeholder_text(Some("Enter characters to exclude")); // Placeholder text for excluded characters
    exclude_entry.set_margin_top(5);
    exclude_entry.set_margin_bottom(5);

    // Checkboxes for character options
    let upper_case_checkbox = CheckButton::with_label("Include Uppercase Letters");
    let lower_case_checkbox = CheckButton::with_label("Include Lowercase Letters");
    let numbers_checkbox = CheckButton::with_label("Include Numbers");
    let special_chars_checkbox = CheckButton::with_label("Include Special Characters");

    // Set default options to true for better user experience
    upper_case_checkbox.set_active(true);
    lower_case_checkbox.set_active(true);
    numbers_checkbox.set_active(true);
    special_chars_checkbox.set_active(true);

    // Button to trigger password generation, with an icon to make it more visually appealing
    let generate_button = Button::new();
    let key_icon = Image::from_icon_name(Some("emblem-keys"), gtk::IconSize::Button); // Icon for the button
    generate_button.set_image(Some(&key_icon)); // Set the icon on the button
    generate_button.set_label("Generate Password"); // Set the button label
    generate_button.style_context().add_class("suggested-action"); // Add a CSS class to style the button
    
    // TextView to display the generated password
    let result_textview = TextView::new();
    result_textview.set_editable(false); // Make the result field read-only so users can't edit it
    result_textview.set_cursor_visible(false); // Hide the cursor for better aesthetics
    result_textview.set_wrap_mode(gtk::WrapMode::Word); // Wrap text at word boundaries
    result_textview.set_margin_top(10);
    result_textview.set_margin_bottom(10);
    result_textview.set_tooltip_text(Some("Generated password (read-only)")); // Add a tooltip to indicate read-only

    // Label to display error messages
    let error_label = Label::new(None);
    error_label.style_context().add_class("error"); // Style the error label for visibility
    error_label.set_margin_bottom(5);

    // Progress bar for password strength
    let strength_bar = ProgressBar::new();
    strength_bar.set_margin_top(10);
    strength_bar.set_margin_bottom(10);
    strength_bar.set_show_text(true); // Display the text indicating the strength level

    // Add all widgets to the vbox container
    vbox.pack_start(&length_entry, false, false, 0); // Add password length entry
    vbox.pack_start(&exclude_entry, false, false, 0); // Add excluded characters entry
    vbox.pack_start(&upper_case_checkbox, false, false, 0); // Add checkbox for uppercase letters
    vbox.pack_start(&lower_case_checkbox, false, false, 0); // Add checkbox for lowercase letters
    vbox.pack_start(&numbers_checkbox, false, false, 0); // Add checkbox for numbers
    vbox.pack_start(&special_chars_checkbox, false, false, 0); // Add checkbox for special characters
    vbox.pack_start(&separator, false, false, 10); // Add separator for visual clarity
    vbox.pack_start(&generate_button, false, false, 0); // Add the generate button
    vbox.pack_start(&error_label, false, false, 0); // Add the error label
    vbox.pack_start(&strength_bar, false, false, 0); // Add the strength progress bar
    let strength_label = Label::new(None); // Label to display password strength as text
    strength_label.set_widget_name("strength-label");
    vbox.pack_start(&strength_label, false, false, 0); // Add the strength label
    vbox.pack_start(&result_textview, true, true, 0); // Add the result TextView

    // Apply a CSS style for a modern look by loading CSS from an external file
    use std::fs;
    let css_path = "src/style.css";
    let css = match fs::read_to_string(css_path) {
        Ok(data) => data,
        Err(_) => {
            eprintln!("Failed to read CSS file. Falling back to default style.");
            return;
        }
    };

    // Load the CSS to style the application
    let provider = CssProvider::new();
    provider.load_from_data(css.as_bytes()).expect("Failed to load CSS");
    if let Some(screen) = gtk::gdk::Screen::default() {
        StyleContext::add_provider_for_screen(&screen, &provider, STYLE_PROVIDER_PRIORITY_USER);
    } else {
        eprintln!("Failed to get default screen. The application may not display styles properly.");
        return;
    }

    // Add the container to the window
    window.add(&vbox);

    // Connect the generate button click event
    generate_button.connect_clicked(move |_| {
        error_label.set_text(""); // Clear previous error message

        // Parse the password length input
        let length = match length_entry.text().parse::<usize>() {
            Ok(l) => l,
            Err(_) => {
                // Display error if the input is invalid
                error_label.set_text("Invalid length. Please enter a valid number.");
                return;
            }
        };

        // Get the excluded characters from the input field
        let excluded_chars = exclude_entry.text().to_string();
        
        // Create a new PasswordGenerator instance
        let mut generator = PasswordGenerator::new();
        generator.set_length(length); // Set the length of the password
        generator.set_excluded_character_set(excluded_chars); // Set characters to be excluded

        // Create a set of character classes based on the user's selection
        let mut char_classes: HashSet<CharClass> = HashSet::new();
        if upper_case_checkbox.is_active() {
            char_classes.insert(CharClass::UpperLetters); // Include uppercase letters if selected
        }
        if lower_case_checkbox.is_active() {
            char_classes.insert(CharClass::LowerLetters); // Include lowercase letters if selected
        }
        if numbers_checkbox.is_active() {
            char_classes.insert(CharClass::Numbers); // Include numbers if selected
        }
        if special_chars_checkbox.is_active() {
            char_classes.insert(CharClass::SpecialCharacters); // Include special characters if selected
        }

        // If no character classes are selected, display an error message
        if char_classes.is_empty() {
            error_label.set_text("Please select at least one character class.");
            return;
        }

        // Set the character classes for the password generator
        generator.set_char_classes(char_classes);

        // Generate the password and display it in the TextView
        let password = generator.generate_password();
        if let Some(buffer) = result_textview.buffer() {
            buffer.set_text(&password); // Set the generated password in the TextView buffer
        } else {
            eprintln!("Failed to get the buffer for the result TextView.");
        }

        // Calculate entropy and password strength
        let entropy = calculate_entropy(&password); // Calculate the entropy of the password
        let strength_score = (entropy / 60.0).clamp(0.0, 1.0); // Normalize the strength score to [0, 1]
        strength_bar.set_fraction(strength_score); // Set the progress bar to represent the strength
        let strength_text = get_strength(entropy); // Get the strength description based on entropy
        strength_bar.set_text(Some(&strength_text)); // Set the strength text on the progress bar
        strength_label.set_text(&strength_text); // Update the strength label
    });

    // Show all widgets in the window
    window.show_all();
}
