use std::fmt::{self, Display, Formatter};

use itertools::Itertools;

use crate::postgres::general::{Condition, OutputExpression, TableName, WithClause};
use crate::tools::IntoIteratorOfSameType;

/// Create a new `DELETE FROM` query with the given table name.
///
/// Returns a [`DeleteFrom`] structure that allows adding additional clauses. Call `to_string` to finalize and get SQL.
///
/// # Supported clauses
///
/// | Clause      | Method                               |
/// |-------------|--------------------------------------|
/// | `WHERE`     | [`where_`][DeleteFrom::where_]       |
/// | `RETURNING` | [`returning`][DeleteFrom::returning] |
///
/// # Specifying a `WITH` clause
///
/// To create a `DELETE FROM` query with a `WITH` clause, start with [`with`][crate::postgres::with] instead of this function.
///
/// # Examples
///
/// ```
/// use scooby::postgres::delete_from;
///
/// let sql = delete_from("Dummy").to_string();
///
/// assert_eq!(sql, "DELETE FROM Dummy");
/// ```
///
/// ```
/// use scooby::postgres::delete_from;
///
/// let sql = delete_from("Dummy")
///     .where_("x > 0")
///     .where_("y < 10")
///     .returning("id")
///     .to_string();
///
/// assert_eq!(sql, "DELETE FROM Dummy WHERE x > 0 AND y < 10 RETURNING id");
/// ```
pub fn delete_from(table_name: impl Into<TableName>) -> DeleteFrom {
    DeleteFrom::new(table_name.into(), None)
}

pub(crate) fn delete_from_with(table_name: TableName, with: WithClause) -> DeleteFrom {
    DeleteFrom::new(table_name, Some(with))
}

/// `DELETE FROM` statement with optional `WHERE` conditions and `RETURNING` clauses.
///
/// Finalize and turn into `String` by calling `to_string`.
///
/// See [`delete_from`] docs for more details and examples.
#[must_use = "Making a DELETE FROM without using it is pointless"]
#[derive(Debug, Clone)]
pub struct DeleteFrom {
    table_name: TableName,
    with: Option<WithClause>,
    where_: Vec<Condition>,
    returning: Vec<OutputExpression>,
}

impl DeleteFrom {
    fn new(table_name: TableName, with: Option<WithClause>) -> DeleteFrom {
        DeleteFrom {
            table_name,
            with,
            where_: Vec::new(),
            returning: Vec::new(),
        }
    }

    /// Add one or more `WHERE` conditions, `AND`'ed together with themselves and existing conditions.
    ///
    /// ```
    /// use scooby::postgres::delete_from;
    ///
    /// let sql = delete_from("Dummy")
    ///     .where_(("x > 1", "y > 1"))
    ///     .where_("z > 1")
    ///     .to_string();
    ///
    /// assert_eq!(sql, "DELETE FROM Dummy WHERE x > 1 AND y > 1 AND z > 1");
    /// ```
    pub fn where_(mut self, conditions: impl IntoIteratorOfSameType<Condition>) -> Self {
        self.where_.extend(conditions.into_some_iter());
        self
    }

    /// Add one or more `RETURNING` expressions.
    ///
    /// ```
    /// use scooby::postgres::delete_from;
    ///
    /// let sql = delete_from("Dummy")
    ///     .returning("id")
    ///     .returning(("width", "height"))
    ///     .to_string();
    ///
    /// assert_eq!(sql, "DELETE FROM Dummy RETURNING id, width, height");
    /// ```
    pub fn returning(mut self, expressions: impl IntoIteratorOfSameType<OutputExpression>) -> Self {
        self.returning.extend(expressions.into_some_iter());
        self
    }
}

impl Display for DeleteFrom {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(with_clause) = &self.with {
            write!(f, "{} ", with_clause)?;
        }

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
    use crate::postgres::tools::tests::assert_correct_postgresql;
    use crate::postgres::{delete_from, select, with};

    #[test]
    fn everything() {
        let sql = delete_from("Dummy").to_string();
        assert_correct_postgresql(&sql, "DELETE FROM Dummy");
    }

    #[test]
    fn where_one() {
        let sql = delete_from("Dummy").where_("x > 0").to_string();
        assert_correct_postgresql(&sql, "DELETE FROM Dummy WHERE x > 0");
    }

    #[test]
    fn where_many() {
        let sql = delete_from("Dummy").where_(("x > 0", "y > 30")).to_string();
        assert_correct_postgresql(&sql, "DELETE FROM Dummy WHERE x > 0 AND y > 30");
    }

    #[test]
    fn where_chain() {
        let sql = delete_from("Dummy")
            .where_("x > 0")
            .where_("y < 10")
            .to_string();

        assert_correct_postgresql(&sql, "DELETE FROM Dummy WHERE x > 0 AND y < 10");
    }

    #[test]
    fn returning() {
        let sql = delete_from("Dummy").returning("id").to_string();
        assert_correct_postgresql(&sql, "DELETE FROM Dummy RETURNING id");
    }

    #[test]
    fn returning_two() {
        let sql = delete_from("Dummy").returning(("id", "place")).to_string();
        assert_correct_postgresql(&sql, "DELETE FROM Dummy RETURNING id, place");
    }

    #[test]
    fn cte() {
        let sql = with("thing")
            .as_(select("1 + 1"))
            .delete_from("Dummy")
            .to_string();

        assert_correct_postgresql(&sql, "WITH thing AS (SELECT 1 + 1) DELETE FROM Dummy");
    }
}
