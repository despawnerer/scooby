use itertools::Itertools;
use std::fmt::Display;

use crate::postgres::general::{Expression, TableName};
use crate::postgres::queries::{select_with, DeleteFrom, InsertInto, Select, Update, Values};
use crate::tools::IntoIteratorOfSameType;

use super::Column;

pub fn with(name: impl Into<TableName>) -> WithQueryBuilder {
    WithQueryBuilder {
        clause: WithClause::new(),
        name: name.into(),
        columns: Vec::new(),
    }
}

#[derive(Debug, Clone, Default)]
pub struct WithClause {
    queries: Vec<WithQuery>,
}

impl WithClause {
    pub fn new() -> WithClause {
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
        select_with(expressions, self)
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
