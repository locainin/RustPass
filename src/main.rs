extern crate rand;
use rand::Rng;
use std::collections::HashSet;
use std::io::{self, Write};
use std::str::FromStr;


pub struct PasswordGenerator {
    pub default_length: usize,
    pub default_custom_character_set: String,
    pub default_excluded_chars: String,
    pub length: usize,
    pub classes: HashSet<CharClass>,
    pub flags: HashSet<GeneratorFlag>,
    pub custom: String,
    //pub excluded: String,
    pub excluded: HashSet<char>,
}


impl PasswordGenerator {
    pub fn new() -> Self {
        let mut excluded = HashSet::new();
        let chars_to_exclude = ['a', 'b', 'c']; // Add more characters as needed
        for &ch in chars_to_exclude.iter() {
            excluded.insert(ch);
        }

        PasswordGenerator {
            default_length: 32,
            default_custom_character_set: "".to_string(),
            default_excluded_chars: "".to_string(),
            length: 32,
            classes: [CharClass::NoClass].iter().cloned().collect(),
            flags: [GeneratorFlag::NoFlags].iter().cloned().collect(),
            custom: "".to_string(),
            excluded
        }
    }

    pub fn num_char_classes(&self) -> usize {
        let non_empty_groups = self.password_groups();
        non_empty_groups.len()
    }

    pub fn get_min_length(&self) -> usize {
        if self.flags.contains(&GeneratorFlag::CharFromEveryGroup) {
            self.num_char_classes()
        } else {
            1
        }
    }

    pub fn reset(&mut self) {
        self.classes = [CharClass::NoClass].iter().cloned().collect();
        self.flags = [GeneratorFlag::NoFlags].iter().cloned().collect();
        self.custom = Default::default();
        self.excluded = Default::default();
        self.length = 32;
    }

    pub fn set_length(&mut self, length: usize) {
        self.length = length;
    }

    pub fn set_char_classes(&mut self, classes: HashSet<CharClass>) {
        self.classes = classes;
    }

    pub fn set_custom_character_set(&mut self, custom_character_set: String) {
        self.custom = custom_character_set;
    }

    pub fn set_excluded_character_set(&mut self, excluded_character_set: String) {
        self.excluded = excluded_character_set.chars().collect();
    }

    pub fn set_flags(&mut self, flags: HashSet<GeneratorFlag>) {
        self.flags = flags;
    }

    pub fn is_valid(&self) -> bool {
        if self.classes.is_empty() && self.custom.is_empty() {
            false
        } else if self.length == 0 {
            false
        } else {
            true
        }
    }

    pub fn password_groups(&self) -> Vec<Vec<char>> {
        let mut password_groups: Vec<Vec<char>> = Vec::new();

        if self.classes.contains(&CharClass::LowerLetters) {
            let lowercase: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
            password_groups.push(lowercase);
        }

        if self.classes.contains(&CharClass::UpperLetters) {
            let uppercase: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
            password_groups.push(uppercase);
        }

        if self.classes.contains(&CharClass::Numbers) {
            let numbers: Vec<char> = "0123456789".chars().collect();
            password_groups.push(numbers);
        }

        if self.classes.contains(&CharClass::SpecialCharacters) {
            let special_characters: Vec<char> = "!@#$%^&*()-_=+[]{}|;:',.<>?/".chars().collect();
            password_groups.push(special_characters);
        }

        for group in password_groups.iter_mut() {
            group.retain(|&ch| !self.excluded.contains(&ch));
        }

        password_groups
    }

    pub fn generate_password(&self) -> String {
        assert!(self.is_valid());
        let groups = self.password_groups();

        //println!("Debug: {:?}", groups);

        let mut password_chars: Vec<char> = Vec::new();
        for group in &groups {


            if !group.is_empty() {  // Check for empty group
                password_chars.extend(group);
            }
        }
        

        let mut password = String::new();
        let mut rng = rand::thread_rng();

        if self.flags.contains(&GeneratorFlag::CharFromEveryGroup) {
            for group in &groups {

                if !group.is_empty() { 


                let pos = rng.gen_range(0..group.len());
                password.push(group[pos]);
            }

        }
    }

        for _ in 0..self.length {
            let pos = rng.gen_range(0..password_chars.len());
            password.push(password_chars[pos]);
        }

        password
    }
}


#[derive(Hash, Eq, PartialEq, Clone)]
pub enum CharClass {
    NoClass,
    LowerLetters,
    UpperLetters,
    Numbers,
    Braces,
    Punctuation,
    Quotes,
    Dashes,
    Math,
    Logograms,
    EASCII,
    SpecialCharacters,
    DefaultCharset,
    Lowercase,  // Add this if it exists in the C++ version
    Uppercase, 
    // Add other character classes
}


#[derive(Hash, Eq, PartialEq, Clone)]
pub enum GeneratorFlag {
    NoFlags,
    ExcludeLookAlike,
    CharFromEveryGroup,
    AdvancedMode,
    DefaultFlags, 
    // Add other generator flags
}

// Add this main function at the bottom of your main.rs

fn main() {
    // Initialize PasswordGenerator
    let mut generator = PasswordGenerator::new();
    
    // Function to flush stdout
    let flush = || io::stdout().flush().unwrap();

    // Prompt user for password length
    loop {
        let mut input = String::new();
        print!("Enter the desired password length: ");
        flush();
        io::stdin().read_line(&mut input).unwrap();
        
        match usize::from_str(input.trim()) {
            Ok(length) => {
                generator.set_length(length);
                break;
            },
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
    println!();

    // Prompt user for characters to exclude
    let mut input = String::new();
    print!("Enter characters to exclude (just type them one after another, no spaces): ");
    flush();
    io::stdin().read_line(&mut input).unwrap();
    generator.set_excluded_character_set(input.trim().to_string());
    println!();

    // Explicitly specify the type for char_classes
    let mut char_classes: HashSet<CharClass> = HashSet::new();

    // Function to prompt for character class
    let prompt_for_class = |char_classes: &mut HashSet<CharClass>, class: CharClass, description: &str| {
        loop {
            let mut input = String::new();
            print!("Include {}? (y/n): ", description);
            flush();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().to_lowercase().as_str() {
                "y" => {
                    char_classes.insert(class);
                    break;
                },
                "n" => break,
                _ => println!("Invalid input. Please enter 'y' or 'n'."),
            }
        }
    };

    // Prompt for various character classes
    prompt_for_class(&mut char_classes, CharClass::UpperLetters, "Uppercase Letters");
    println!();
    prompt_for_class(&mut char_classes, CharClass::LowerLetters, "Lowercase Letters");
    println!();
    prompt_for_class(&mut char_classes, CharClass::Numbers, "Numbers");
    println!();
    prompt_for_class(&mut char_classes, CharClass::SpecialCharacters, "Special Characters");
    println!();

    // Update the generator with the chosen character classes
    generator.set_char_classes(char_classes);

    // Generate a password
    let password = generator.generate_password();
    println!("Generated password: {}", password);
}

