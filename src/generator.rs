
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

    /// Generate a password using the configured options. If after applying
    /// the exclusion list no characters remain in the pool this will return an
    /// error instead of panicking.
    pub fn generate_password(&self) -> Result<String, &'static str> {
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

        if character_pool.is_empty() {
            return Err("No characters available after exclusions");
        }

        // Generate password of specified length
        let mut rng = rand::thread_rng();
        let password: String = (0..self.length)
            .map(|_| {
                let idx = rng.gen_range(0..character_pool.len());
                character_pool
                    .chars()
                    .nth(idx)
                    .expect("index within bounds because character_pool isn't empty")
            })
            .collect();

        Ok(password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn generate_password_empty_pool() {
        let mut gen = PasswordGenerator::new();
        gen.set_length(8);
        gen.set_char_classes(HashSet::from_iter([CharClass::Numbers]));
        gen.set_excluded_character_set("0123456789".to_string());
        assert!(gen.generate_password().is_err());
    }

    #[test]
    fn generate_password_success() {
        let mut gen = PasswordGenerator::new();
        gen.set_length(4);
        gen.set_char_classes(HashSet::from_iter([CharClass::LowerLetters]));
        let pwd = gen.generate_password().unwrap();
        assert_eq!(pwd.len(), 4);
    }
}