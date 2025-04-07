pub fn remove_letter_sensitive(s: &str, letter: char) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if c != letter {
            result.push(c);
        }
    }
    result
}

pub fn remove_letter_insensitive(s: &str, letter: char) -> String {
    let mut result = String::new();
    let target = letter.to_ascii_lowercase();
    for c in s.chars() {
        if c.to_ascii_lowercase() != target {
            result.push(c);
        }
    }
    result
}

pub fn swap_letter_case(s: &str, letter: char) -> String {
    let mut result = String::new();
    let target = letter.to_ascii_lowercase();
    for c in s.chars() {
        if c.to_ascii_lowercase() == target {
            if c.is_lowercase() {
                result.push(c.to_ascii_uppercase());
            } else {
                result.push(c.to_ascii_lowercase());
            }
        } else {
            result.push(c);
        }
    }
    result
}
