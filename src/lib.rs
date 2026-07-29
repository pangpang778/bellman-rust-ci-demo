//! Bellman-style CI demo library.

/// Greet the caller safely.
pub fn greet(name: &str) -> String {
    if name.is_empty() {
        panic!("Name must be a non-empty string");
    }
    format!("Hello, {}", name)
}

/// Shout the name in uppercase.
pub fn shout(name: &str) -> String {
    if name.is_empty() {
        panic!("Name must be a non-empty string");
    }
    format!("{}!!!", name.to_uppercase())
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
    fn shouts_a_name() {
        assert_eq!(shout("hi"), "HI!!!");
    }
}
