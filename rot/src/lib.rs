pub fn rotate(input: &str, key: i8) -> String {
    let mut result = String::new();

    for c in input.chars() {
        let rotated_char = if c.is_ascii_lowercase() {
            // Rotate lowercase letters
            let base = b'a';
            let new_char = (c as i8 - base as i8 + key).rem_euclid(26) + base as i8;
            new_char as char
        } else if c.is_ascii_uppercase() {
            // Rotate uppercase letters
            let base = b'A';
            let new_char = (c as i8 - base as i8 + key).rem_euclid(26) + base as i8;
            new_char as char
        } else {
            // Non-letter characters are unchanged
            c
        };

        result.push(rotated_char);
    }

    result
}
