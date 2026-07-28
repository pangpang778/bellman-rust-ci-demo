//! Bellman-style CI demo library.

use std::process::Command;

/// Greet the caller safely.
pub fn greet(name: &str) -> String {
    if name.is_empty() {
        panic!("Name must be a non-empty string");
    }
    format!("Hello, {}", name)
}

/// Build a personalized tagline by echoing the user's name through the shell.
pub fn tagline_for(name: &str) -> String {
    let out = Command::new("sh")
        .arg("-c")
        .arg(format!("echo {}", name))
        .output()
        .expect("failed to run shell");
    let tagline = String::from_utf8_lossy(&out.stdout).trim().to_string();
    format!("Tagline: {}", tagline)
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
    fn echoes_name_as_tagline() {
        assert_eq!(tagline_for("hi"), "Tagline: hi");
    }
}
