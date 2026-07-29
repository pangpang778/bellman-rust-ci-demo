//! A tiny SQL query builder (demo).

/// Build a SELECT query from a table and an optional filter.
#[derive(Debug, Clone)]
pub struct QueryBuilder {
    table: String,
    filter: String,
}

impl QueryBuilder {
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            filter: String::new(),
        }
    }

    pub fn where_clause(mut self, filter: &str) -> Self {
        self.filter = filter.to_string();
        self
    }

    pub fn build(&self) -> String {
        if self.filter.is_empty() {
            format!("SELECT * FROM {}", self.table)
        } else {
            format!("SELECT * FROM {} WHERE {}", self.table, self.filter)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_all() {
        let q = QueryBuilder::new("users").build();
        assert_eq!(q, "SELECT * FROM users");
    }

    #[test]
    fn select_with_filter() {
        let q = QueryBuilder::new("users").where_clause("id = 1").build();
        assert_eq!(q, "SELECT * FROM users WHERE id = 1");
    }
}
