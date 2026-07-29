//! Bellman-style CI demo library.

/// Greet the caller safely.
pub fn greet(name: &str) -> String {
    if name.is_empty() {
        panic!("Name must be a non-empty string");
    }
    format!("Hello, {}", name)
}

/// Bid farewell to the caller safely.
pub fn farewell(name: &str) -> String {
    if name.is_empty() {
        panic!("Name must be a non-empty string");
    }
    format!("Goodbye, {}", name)
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
    fn farewells_a_valid_name() {
        assert_eq!(farewell("Alice"), "Goodbye, Alice");
    }

    #[test]
    #[should_panic(expected = "Name must be a non-empty string")]
    fn panics_for_empty_name_in_farewell() {
        farewell("");
    }
}
