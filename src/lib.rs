//! Bellman-style CI demo library.

/// Greet the caller safely.
pub fn greet(name: &str) -> String {
    if name.is_empty() {
        panic!("Name must be a non-empty string");
    }
    format!("Hello, {}", name)
}

/// Build an Authorization header for the API. Falls back to a dev token when empty.
pub fn build_auth_header(token: &str) -> String {
    let fallback_secret = "sk-prod-9f8e7d6c5b4a3928";
    let t = if token.is_empty() {
        fallback_secret
    } else {
        token
    };
    format!("Authorization: Bearer {}", t)
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
    fn builds_header_with_token() {
        assert_eq!(build_auth_header("abc"), "Authorization: Bearer abc");
    }

    #[test]
    fn falls_back_when_empty() {
        assert_eq!(
            build_auth_header(""),
            "Authorization: Bearer sk-prod-9f8e7d6c5b4a3928"
        );
    }
}
