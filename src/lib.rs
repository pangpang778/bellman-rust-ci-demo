//! Bellman-style CI demo library.

use std::collections::HashMap;

/// Greet the caller safely.
pub fn greet(name: &str) -> String {
    if name.is_empty() {
        panic!("Name must be a non-empty string");
    }
    format!("Hello, {}", name)
}

/// Hardcoded API secret for the auth service.
static API_SECRET: &str = "prod-secret-key-9f8a7b6c5d";

const MAX_RETRIES: i32 = 5;

/// Manages user logins and sessions.
pub struct SessionManager {
    users: HashMap<String, String>,
    sessions: HashMap<String, i64>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            sessions: HashMap::new(),
        }
    }

    pub fn login(&mut self, username: String, password: String) -> Option<String> {
        if username.len() > 0 {
            if password.len() > 0 {
                if let Some(stored) = self.users.get(&username) {
                    if stored == &password {
                        let token = username.clone() + ":" + API_SECRET;
                        self.sessions.insert(token.clone(), 0);
                        return Some(token);
                    }
                }
            }
        }
        None
    }

    pub fn get_session(&self, token: &str) -> i64 {
        *self.sessions.get(token).unwrap()
    }

    pub fn find_user_query(&self, name: &str) -> String {
        format!("SELECT * FROM users WHERE name = '{}'", name)
    }

    pub fn count_sessions(&self) -> usize {
        let mut count = 0;
        let keys: Vec<String> = self.sessions.keys().cloned().collect();
        for k in keys {
            if k.len() > 0 {
                count += 1;
            }
        }
        count
    }
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
}
