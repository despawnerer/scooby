use std::fmt::{self, Display, Formatter};

use itertools::Itertools;

use crate::postgres::general::{
    Column, Condition, Expression, OutputExpression, TableName, WithClause,
};
use crate::tools::IntoIteratorOfSameType;

/// Start building a new `UPDATE` statement with the given table name.
///
/// Returns a [`BareUpdate`] structure that requires that you at least set one
/// column + expression pair through [`set`][BareUpdate::set] method.
///
/// The `set` method, in turn, returns an [`Update`] structure, through which
/// more values may be set, and additional clauses may be added.
///
/// Call `to_string` on the `Update` structure to finalize and get an SQL string.
///
/// # Supported clauses
///
/// | Clause      | Method                           |
/// |-------------|----------------------------------|
/// | `SET`       | [`set`][Update::set]             |
/// | `WHERE`     | [`where_`][Update::where_]       |
/// | `RETURNING` | [`returning`][Update::returning] |
///
/// # Specifying a `WITH` clause
///
/// To create a `UPDATE` statement with a `WITH` clause, start with [`with`][crate::postgres::with] instead of this function.
///
/// # Examples
///
/// ```
/// use scooby::postgres::update;
///
/// let sql = update("Dummy").set("x", 1).to_string();
///
/// assert_eq!(sql, "UPDATE Dummy SET x = 1");
/// ```
///
/// ```
/// use scooby::postgres::update;
///
/// let sql = update("Dummy")
///     .set("x", 1)
///     .where_("x > 0")
///     .where_("y < 10")
///     .returning("id")
///     .to_string();
///
/// assert_eq!(sql, "UPDATE Dummy SET x = 1 WHERE x > 0 AND y < 10 RETURNING id");
/// ```
pub fn update(table_name: impl Into<TableName>) -> BareUpdate {
    BareUpdate {
        table_name: table_name.into(),
        with: None,
    }
}

pub(crate) fn update_with(table_name: TableName, with: WithClause) -> BareUpdate {
    BareUpdate {
        table_name,
        with: Some(with),
    }
}

/// Bare `UPDATE` statement without a `SET` clause specified
///
/// You will want to use the [`set`][BareUpdate::set] method add a `SET` clause
/// with a column + expression pair and turn this into a usable query.
#[must_use = "Making an UPDATE query with no values set is pointless"]
#[derive(Debug)]
pub struct BareUpdate {
    table_name: TableName,
    with: Option<WithClause>,
}

impl BareUpdate {
    /// Add a `SET` clause to the statement with the first column + expression pair.
    ///
    /// Further values can be added through [`set`][Update::set] method on the returned [`Update`] structure.
    ///
    /// ```
    /// use scooby::postgres::update;
    ///
    /// let sql = update("Dummy").set("x", 1).to_string();
    ///
    /// assert_eq!(sql, "UPDATE Dummy SET x = 1");
    /// ```
    pub fn set(self, column: impl Into<Column>, value: impl Into<Expression>) -> Update {
        Update::new(
            self.table_name,
            vec![(column.into(), value.into())],
            self.with,
        )
    }
}

/// `UPDATE` statement with at least one set of values, and possibly additional clauses.
///
/// Finalize and turn into `String` by calling `to_string`.
///
/// See [`update`] docs for more details and examples.
#[must_use = "Making an UPDATE query without using it is pointless"]
#[derive(Debug, Clone)]
pub struct Update {
    table_name: TableName,
    with: Option<WithClause>,
    values: Vec<(Column, Expression)>,
    where_: Vec<Condition>,
    returning: Vec<OutputExpression>,
}

impl Update {
    fn new(
        table_name: TableName,
        values: Vec<(Column, Expression)>,
        with: Option<WithClause>,
    ) -> Update {
        Update {
            table_name,
            values,
            with,
            where_: Vec::new(),
            returning: Vec::new(),
        }
    }

    /// Add a column + expression pair to the `SET` clause of this statement
    ///
    /// ```
    /// use scooby::postgres::update;
    ///
    /// let sql = update("Dummy").set("x", 1).set("y", 2).to_string();
    ///
    /// assert_eq!(sql, "UPDATE Dummy SET x = 1, y = 2");
    /// ```
    pub fn set(mut self, column: impl Into<Column>, value: impl Into<Expression>) -> Self {
        self.values.push((column.into(), value.into()));
        self
    }

    /// Add one or more `WHERE` conditions, `AND`'ed together with themselves and existing conditions.
    ///
    /// ```
    /// use scooby::postgres::update;
    ///
    /// let sql = update("Dummy")
    ///     .set("x", 1)
    ///     .where_(("x > 1", "y > 1"))
    ///     .where_("z > 1")
    ///     .to_string();
    ///
    /// assert_eq!(sql, "UPDATE Dummy SET x = 1 WHERE x > 1 AND y > 1 AND z > 1");
    /// ```
    pub fn where_(mut self, conditions: impl IntoIteratorOfSameType<Condition>) -> Self {
        self.where_.extend(conditions.into_some_iter());
        self
    }

    /// Add one or more `RETURNING` expressions.
    ///
    /// ```
    /// use scooby::postgres::update;
    ///
    /// let sql = update("Dummy")
    ///     .set("x", 1)
    ///     .returning("id")
    ///     .returning(("width", "height"))
    ///     .to_string();
    ///
    /// assert_eq!(sql, "UPDATE Dummy SET x = 1 RETURNING id, width, height");
    /// ```
    pub fn returning(mut self, expressions: impl IntoIteratorOfSameType<OutputExpression>) -> Self {
        self.returning.extend(expressions.into_some_iter());
        self
    }
}

impl Display for Update {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(with_clause) = &self.with {
            write!(f, "{} ", with_clause)?;
        }

        write!(
            f,
            "UPDATE {} SET {}",
            self.table_name,
            self.values
                .iter()
                .map(|(col, val)| format!("{} = {}", col, val))
                .join(", ")
        )?;

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
    use crate::postgres::{select, update, with};

    #[test]
    fn update_single_value() {
        let sql = update("Dummy").set("x", "y").to_string();
        assert_correct_postgresql(&sql, "UPDATE Dummy SET x = y");
    }

    #[test]
    fn update_multi_call() {
        let sql = update("Dummy").set("x", "y").set("a", "b").to_string();
        assert_correct_postgresql(&sql, "UPDATE Dummy SET x = y, a = b");
    }

    #[test]
    fn update_where() {
        let sql = update("Dummy").set("x", "y").where_("id = 5").to_string();
        assert_correct_postgresql(&sql, "UPDATE Dummy SET x = y WHERE id = 5");
    }

    #[test]
    fn update_returning() {
        let sql = update("Dummy").set("x", "y").returning("x").to_string();
        assert_correct_postgresql(&sql, "UPDATE Dummy SET x = y RETURNING x");
    }

    #[test]
    fn cte() {
        let sql = with("thing")
            .as_(select("1 + 1"))
            .update("Dummy")
            .set("x", "y")
            .to_string();

        assert_correct_postgresql(&sql, "WITH thing AS (SELECT 1 + 1) UPDATE Dummy SET x = y");
    }
}
