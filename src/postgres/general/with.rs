use itertools::Itertools;
use std::fmt::Display;

use crate::postgres::general::{Expression, TableName};
use crate::postgres::queries::{
    delete_from_with, insert_into_with, select_with, update_with, BareInsertInto, BareUpdate,
    DeleteFrom, InsertInto, Select, Update, Values,
};
use crate::tools::IntoIteratorOfSameType;

use super::Column;

/// Start a new `WITH` clause for Common Table Expressions
pub fn with(name: impl Into<TableName>) -> WithQueryBuilder {
    WithQueryBuilder {
        clause: WithClause::new(),
        name: name.into(),
        columns: Vec::new(),
    }
}

/// `WITH` clause usable with different types of queries
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

    pub fn is_empty(&self) -> bool {
        self.queries.is_empty()
    }

    pub fn and(self, name: impl Into<TableName>) -> WithQueryBuilder {
        WithQueryBuilder {
            clause: self,
            name: name.into(),
            columns: Vec::new(),
        }
    }

    pub fn select(self, expressions: impl IntoIteratorOfSameType<Expression>) -> Select {
        select_with(expressions.into_some_iter().collect(), self)
    }

    pub fn delete_from(self, table_name: impl Into<TableName>) -> DeleteFrom {
        delete_from_with(table_name.into(), self)
    }

    pub fn update(self, table_name: impl Into<TableName>) -> BareUpdate {
        update_with(table_name.into(), self)
    }

    pub fn insert_into(self, table_name: impl Into<TableName>) -> BareInsertInto {
        insert_into_with(table_name.into(), self)
    }
}

impl Display for WithClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WITH {}",
            self.queries.iter().map(WithQuery::to_string).join(", ")
        )
    }
}

#[derive(Debug, Clone)]
pub struct WithQuery {
    name: TableName,
    columns: Vec<Column>,
    as_: String,
}

impl Display for WithQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;

        if self.columns.len() > 0 {
            write!(f, "({})", self.columns.iter().join(", "))?;
        }

        write!(f, " AS ({})", self.as_)?;

        Ok(())
    }
}

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

    pub fn as_(mut self, target: impl UsableInWithQuery) -> WithClause {
        self.clause.queries.push(WithQuery {
            name: self.name,
            columns: self.columns,
            as_: target.to_string(),
        });
        self.clause
    }
}

pub trait UsableInWithQuery: Display {}

impl UsableInWithQuery for Select {}
impl<V: Values> UsableInWithQuery for InsertInto<V> {}
impl UsableInWithQuery for Update {}
impl UsableInWithQuery for DeleteFrom {}
