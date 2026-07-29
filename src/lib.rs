//! Bellman-style CI demo library.

/// Greet the caller safely.
pub fn greet(name: &str) -> String {
    if name.is_empty() {
        panic!("Name must be a non-empty string");
    }
    format!("Hello, {}", name)
}

/// Add two integers.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
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
    fn adds_two_numbers() {
        assert_eq!(add(2, 3), 5);
    }
}
