// src/calc_password_strength.rs

use std::collections::HashSet;


pub fn calculate_entropy(password: &str) -> f64 {
    if password.is_empty() {
        return 0.0; // Return 0 for empty password
    }

    let charset = calculate_charset(password);
    let length = password.len();

    length as f64 * charset.log2()
}

fn calculate_charset(password: &str) -> f64 {
    if password.is_empty() {
        return 0.0; // Return 0 for empty password
    }

    let mut char_types: HashSet<&str> = HashSet::new();

    for byte in password.bytes() {
        if byte >= b'0' && byte <= b'9' {
            char_types.insert("numbers");
        } else if byte >= b'a' && byte <= b'z' {
            char_types.insert("lowercase");
        } else if byte >= b'A' && byte <= b'Z' {
            char_types.insert("uppercase");
        } else {
            char_types.insert("special");
        }
    }

    let charset_size = char_types.iter().fold(0, |acc, &char_type| {
        acc + match char_type {
            "numbers" => 10,
            "lowercase" => 26,
            "uppercase" => 26,
            "special" => 33,
            _ => 0,
        }
    });

    charset_size as f64
}

pub fn get_strength(entropy: f64) -> String {
    match entropy {
        e if e < 28.0 => "Weak".to_string(),
        e if e < 36.0 => "Moderate".to_string(),
        e if e < 60.0 => "Strong".to_string(),
        _ => "Very Strong".to_string(),
    }
}
