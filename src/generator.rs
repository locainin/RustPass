
use std::collections::HashSet;
use crate::utils::CharClass;
use rand::Rng;

pub struct PasswordGenerator {
    pub length: usize,
    pub excluded_character_set: String,
    pub char_classes: HashSet<CharClass>,
}

impl PasswordGenerator {
    pub fn new() -> Self {
        PasswordGenerator {
            length: 12, // Default length
            excluded_character_set: String::new(),
            char_classes: HashSet::new(),
        }
    }

    pub fn set_length(&mut self, length: usize) {
        self.length = length;
    }

    pub fn set_excluded_character_set(&mut self, excluded_chars: String) {
        self.excluded_character_set = excluded_chars;
    }

    pub fn set_char_classes(&mut self, char_classes: HashSet<CharClass>) {
        self.char_classes = char_classes;
    }

    pub fn generate_password(&self) -> String {
        let mut character_pool = String::new();

        // Add character sets based on selected character classes
        if self.char_classes.contains(&CharClass::UpperLetters) {
            character_pool.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        }
        if self.char_classes.contains(&CharClass::LowerLetters) {
            character_pool.push_str("abcdefghijklmnopqrstuvwxyz");
        }
        if self.char_classes.contains(&CharClass::Numbers) {
            character_pool.push_str("0123456789");
        }
        if self.char_classes.contains(&CharClass::SpecialCharacters) {
            character_pool.push_str("!@#$%^&*()_+-=[]{}|;:',.<>?/");
        }

        // Remove excluded characters from the pool
        for c in self.excluded_character_set.chars() {
            character_pool = character_pool.replace(c, "");
        }

        // Generate password of specified length
        let mut rng = rand::thread_rng();
        let password: String = (0..self.length)
            .map(|_| {
                let idx = rng.gen_range(0..character_pool.len());
                character_pool.chars().nth(idx).unwrap()
            })
            .collect();

        password
    }
}