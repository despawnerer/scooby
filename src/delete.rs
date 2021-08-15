use std::fmt::{self, Display, Formatter};

use itertools::Itertools;

use crate::general::{Condition, OutputExpression};
use crate::tools::IntoSomeIterator;

pub fn delete_from(table_name: &str) -> DeleteFrom {
    DeleteFrom {
        table_name: table_name.to_string(),
        ..Default::default()
    }
}

#[derive(Default, Debug)]
pub struct DeleteFrom {
    table_name: String,
    where_: Vec<Condition>,
    returning: Vec<OutputExpression>,
}

impl DeleteFrom {
    pub fn where_(mut self, conditions: impl IntoSomeIterator<Condition>) -> Self {
        self.where_.extend(conditions.into_some_iter());
        self
    }

    pub fn returning(mut self, expressions: impl IntoSomeIterator<OutputExpression>) -> Self {
        self.returning.extend(expressions.into_some_iter());
        self
    }
}

impl Display for DeleteFrom {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "DELETE FROM {}", self.table_name,)?;

        if self.where_.len() > 0 {
            write!(f, " WHERE {}", self.where_.iter().join(" AND "))?;
        }

        if self.returning.len() > 0 {
            write!(f, " RETURNING {}", self.returning.iter().join(", "))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::delete_from;

    #[test]
    fn everything() {
        let sql = delete_from("Dummy").to_string();
        assert_eq!(sql, "DELETE FROM Dummy");
    }

    #[test]
    fn where_one() {
        let sql = delete_from("Dummy").where_("x > 0").to_string();
        assert_eq!(sql, "DELETE FROM Dummy WHERE x > 0");
    }

    #[test]
    fn where_many() {
        let sql = delete_from("Dummy").where_(("x > 0", "y > 30")).to_string();
        assert_eq!(sql, "DELETE FROM Dummy WHERE x > 0 AND y > 30");
    }

    #[test]
    fn where_chain() {
        let sql = delete_from("Dummy")
            .where_("x > 0")
            .where_("y < 10")
            .to_string();
        assert_eq!(sql, "DELETE FROM Dummy WHERE x > 0 AND y < 10");
    }

    #[test]
    fn returning() {
        let sql = delete_from("Dummy").returning("id").to_string();
        assert_eq!(sql, "DELETE FROM Dummy RETURNING id");
    }

    #[test]
    fn returning_two() {
        let sql = delete_from("Dummy").returning(("id", "place")).to_string();
        assert_eq!(sql, "DELETE FROM Dummy RETURNING id, place");
    }
}
