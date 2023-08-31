use std::fmt::{self, Display, Formatter};

use crate::postgres::general::ColumnValuePair;
use crate::tools::{joined, IntoIteratorOfSameType};

use super::{InsertInto, Values};

#[derive(Debug, Clone)]
pub struct OnConflictClause {
    action: ConflictAction,
}

impl Display for OnConflictClause {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ON CONFLICT {}", self.action)
    }
}

#[derive(Debug, Clone)]
pub enum ConflictAction {
    DoNothing,
    DoUpdateSet(Vec<ColumnValuePair>),
}

impl Display for ConflictAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::DoNothing => write!(f, "DO NOTHING"),
            Self::DoUpdateSet(pairs) => write!(f, "DO UPDATE SET {}", joined(pairs, ", ")),
        }
    }
}

/// Intermediate structure to ensure that an action for an `ON CONFLICT` clause is specified:
///
/// - [`do_nothing`][OnConflictClauseBuilder::do_nothing] to add a `DO NOTHING` action
/// - [`do_update_set`][OnConflictClauseBuilder::do_update_set] to add `DO UPDATE SET ...` action
pub struct OnConflictClauseBuilder<V: Values> {
    statement: InsertInto<V>,
}

impl<V: Values> OnConflictClauseBuilder<V> {
    pub(crate) fn new(statement: InsertInto<V>) -> Self {
        Self { statement }
    }

    /// Add a `DO NOTHING` action to this `ON CONFLICT` clause.
    ///
    /// Returns back to the [`InsertInto`] statement.
    ///
    /// ```
    /// use scooby::postgres::insert_into;
    ///
    /// let sql = insert_into("Dummy")
    ///     .values(["a"])
    ///     .on_conflict()
    ///     .do_nothing()
    ///     .to_string();
    ///
    /// assert_eq!(sql, "INSERT INTO Dummy VALUES (a) ON CONFLICT DO NOTHING");
    /// ```
    pub fn do_nothing(self) -> InsertInto<V> {
        let mut statement = self.statement;

        statement.on_conflict = Some(OnConflictClause {
            action: ConflictAction::DoNothing,
        });

        statement
    }

    /// Add a `DO UPDATE SET` action to this `ON CONFLICT` clause
    ///
    /// Returns back to the [`InsertInto`] statement.
    ///
    /// ```
    /// use scooby::postgres::insert_into;
    ///
    /// let sql = insert_into("Dummy")
    ///     .values(["a"])
    ///     .on_conflict()
    ///     .do_update_set([("b", "c")])
    ///     .to_string();
    ///
    /// assert_eq!(sql, "INSERT INTO Dummy VALUES (a) ON CONFLICT DO UPDATE SET b = c");
    /// ```
    pub fn do_update_set(
        self,
        pairs: impl IntoIteratorOfSameType<ColumnValuePair>,
    ) -> InsertInto<V> {
        let mut statement = self.statement;

        let cols = pairs.into_some_iter().collect();

        statement.on_conflict = Some(OnConflictClause {
            action: ConflictAction::DoUpdateSet(cols),
        });

        statement
    }
}
