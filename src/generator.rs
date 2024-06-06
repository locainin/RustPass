extern crate rand;
use rand::Rng;
use std::collections::HashSet;
use crate::utils::{CharClass, GeneratorFlag};

pub struct PasswordGenerator {
    pub default_length: usize,
    pub default_custom_character_set: String,
    pub default_excluded_chars: String,
    pub length: usize,
    pub classes: HashSet<CharClass>,
    pub flags: HashSet<GeneratorFlag>,
    pub custom: String,
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

    pub fn set_length(&mut self, length: usize) {
        self.length = length;
    }

    pub fn set_char_classes(&mut self, classes: HashSet<CharClass>) {
        self.classes = classes;
    }

    pub fn set_excluded_character_set(&mut self, excluded: String) {
        self.excluded = excluded.chars().collect();
    }

    pub fn generate_password(&self) -> String {
        let mut rng = rand::thread_rng();
        let mut password = String::with_capacity(self.length);

        let available_chars: Vec<char> = self.get_available_chars();
        for _ in 0..self.length {
            let idx = rng.gen_range(0..available_chars.len());
            password.push(available_chars[idx]);
        }
        password
    }

    fn get_available_chars(&self) -> Vec<char> {
        let mut chars = Vec::new();

        if self.classes.contains(&CharClass::UpperLetters) {
            chars.extend('A'..='Z');
        }
        if self.classes.contains(&CharClass::LowerLetters) {
            chars.extend('a'..='z');
        }
        if self.classes.contains(&CharClass::Numbers) {
            chars.extend('0'..='9');
        }
        if self.classes.contains(&CharClass::SpecialCharacters) {
            chars.extend("!@#$%^&*()".chars());
        }

        chars.retain(|ch| !self.excluded.contains(ch));
        chars
    }
}

