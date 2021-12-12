use std::fmt::Display;

use crate::postgres::general::{Expression, TableName};
use crate::postgres::statements::{
    delete_from_with, insert_into_with, select_with, update_with, BareInsertInto, BareUpdate,
    DeleteFrom, InsertInto, Select, Update, Values,
};
use crate::tools::{joined, IntoIteratorOfSameType};

use super::Column;

/// Start a new `WITH` clause for Common Table Expressions
pub fn with(name: impl Into<TableName>) -> WithQueryBuilder {
    WithQueryBuilder {
        clause: WithClause::new(),
        name: name.into(),
        columns: Vec::new(),
    }
}

/// `WITH` clause usable with different types of statements
///
/// Use [`and_with`][WithClause::and_with] to add a table to the clause.
///
/// Use one of four finalizing methods to start building the actual statement with this clause:
///
/// - [`select`][WithClause::select]
/// - [`delete_from`][WithClause::delete_from]
/// - [`update`][WithClause::update]
/// - [`insert_into`][WithClause::insert_into]
///
/// See [`with`] docs for more details and examples.
#[derive(Debug, Clone, Default)]
pub struct WithClause {
    queries: Vec<WithQuery>,
}

impl WithClause {
    fn new() -> WithClause {
        WithClause {
            queries: Vec::new(),
        }
    }

    /// Add another table under the given name
    pub fn and_with(self, name: impl Into<TableName>) -> WithQueryBuilder {
        WithQueryBuilder {
            clause: self,
            name: name.into(),
            columns: Vec::new(),
        }
    }

    /// Start building a `SELECT` statement with this `WITH` clause
    pub fn select(self, expressions: impl IntoIteratorOfSameType<Expression>) -> Select {
        select_with(expressions.into_some_iter().collect(), self)
    }

    /// Start building a `DELETE FROM` statement with this `WITH` clause
    pub fn delete_from(self, table_name: impl Into<TableName>) -> DeleteFrom {
        delete_from_with(table_name.into(), self)
    }

    /// Start building a `UPDATE` statement with this `WITH` clause
    pub fn update(self, table_name: impl Into<TableName>) -> BareUpdate {
        update_with(table_name.into(), self)
    }

    /// Start building a `INSERT INTO` statement with this `WITH` clause
    pub fn insert_into(self, table_name: impl Into<TableName>) -> BareInsertInto {
        insert_into_with(table_name.into(), self)
    }
}

impl Display for WithClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WITH {}", joined(&self.queries, ", "))
    }
}

/// Specific table inside a `WITH` clause
#[derive(Debug, Clone)]
pub struct WithQuery {
    name: TableName,
    columns: Vec<Column>,
    as_: String,
}

impl Display for WithQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;

        if !self.columns.is_empty() {
            write!(f, "({})", joined(&self.columns, ", "))?;
        }

        write!(f, " AS ({})", self.as_)?;

        Ok(())
    }
}

/// An intermediate structure ensuring that you specify the definition of the table in the `WITH` clause
#[derive(Debug)]
pub struct WithQueryBuilder {
    clause: WithClause,
    name: TableName,
    columns: Vec<Column>,
}

impl WithQueryBuilder {
    pub fn columns(mut self, columns: impl IntoIteratorOfSameType<Column>) -> WithQueryBuilder {
        self.columns.extend(columns.into_some_iter());
        self
    }

    /// Specify the statement that will be used as a basis for the table
    pub fn as_(mut self, target: impl UsableInWithQuery) -> WithClause {
        self.clause.queries.push(WithQuery {
            name: self.name,
            columns: self.columns,
            as_: target.to_string(),
        });
        self.clause
    }
}

/// Marker trait for statements that can be specified inside a `WITH` clause
///
/// - `SELECT`
/// - `INSERT INTO`
/// - `DELETE FROM`
/// - `UPDATE`
pub trait UsableInWithQuery: Display {}

impl UsableInWithQuery for Select {}
impl<V: Values> UsableInWithQuery for InsertInto<V> {}
impl UsableInWithQuery for Update {}
impl UsableInWithQuery for DeleteFrom {}
