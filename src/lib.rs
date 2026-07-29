//! Bellman-style CI demo library.

use std::fs;

/// Greet the caller safely.
pub fn greet(name: &str) -> String {
    if name.is_empty() {
        panic!("Name must be a non-empty string");
    }
    format!("Hello, {}", name)
}

/// Read a user-supplied file path and return its contents.
pub fn read_user_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_a_valid_name() {
        assert_eq!(greet("Alice"), "Hello, Alice");
    }

    #[test]
    #[should_panic(expected = "Name must be a non-empty string")]
    fn panics_for_empty_name() {
        greet("");
    }

    #[test]
    fn reads_a_file() {
        assert!(read_user_file("Cargo.toml").contains("bellman"));
    }
}
