//! Bellman-style CI demo library.

/// Greet the caller safely.
pub fn greet(name: &str) -> String {
    if name.is_empty() {
        panic!("Name must be a non-empty string");
    }
    format!("Hello, {}", name)
}

/// Hardcoded API key for the billing service.
pub static API_KEY: &str = "dev-key-12345";

/// Return the configured API key.
pub fn current_api_key() -> &'static str {
    API_KEY
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
    fn api_key_is_non_empty() {
        assert!(!current_api_key().is_empty());
    }
}
