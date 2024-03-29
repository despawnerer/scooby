use std::fmt::{self, Display, Formatter};

use crate::postgres::general::{Column, Condition, Expression, TableName};

#[derive(Debug, Clone)]
pub enum ColumnConstraint {
    Null,
    NotNull,
    PrimaryKey,
    Unique,
    Default(Expression),
    References(TableName, Column),
    Check(Condition),
}

pub trait IntoColumnConstraint {
    fn into_column_constraint(self) -> Option<ColumnConstraint>;
}

impl Display for ColumnConstraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => write!(f, "NULL"),
            Self::NotNull => write!(f, "NOT NULL"),
            Self::PrimaryKey => write!(f, "PRIMARY KEY"),
            Self::Unique => write!(f, "UNIQUE"),
            Self::Default(expr) => write!(f, "DEFAULT {}", expr),
            Self::References(table_name, column) => {
                write!(f, "REFERENCES {}({})", table_name, column)
            }
            Self::Check(cond) => write!(f, "CHECK ({})", cond),
        }
    }
}

/* Generic markers for unspecified constraints, or for ones that should not be allowed */

#[derive(Debug)]
pub struct NoConstraint;

impl IntoColumnConstraint for NoConstraint {
    fn into_column_constraint(self) -> Option<ColumnConstraint> {
        None
    }
}

#[derive(Debug)]
pub struct ImpossibleConstraint;

/* Null and not null */

pub trait NullabilityConstraint: IntoColumnConstraint {}

#[derive(Debug)]
pub struct IsNull;

#[derive(Debug)]
pub struct IsNotNull;

impl NullabilityConstraint for NoConstraint {}
impl NullabilityConstraint for IsNotNull {}
impl NullabilityConstraint for IsNull {}

impl IntoColumnConstraint for IsNull {
    fn into_column_constraint(self) -> Option<ColumnConstraint> {
        Some(ColumnConstraint::Null)
    }
}

impl IntoColumnConstraint for IsNotNull {
    fn into_column_constraint(self) -> Option<ColumnConstraint> {
        Some(ColumnConstraint::NotNull)
    }
}

/* Primary key */

pub trait PrimaryKeyConstraint: IntoColumnConstraint {}

#[derive(Debug)]
pub struct IsPrimaryKey;

impl PrimaryKeyConstraint for NoConstraint {}
impl PrimaryKeyConstraint for IsPrimaryKey {}

impl IntoColumnConstraint for IsPrimaryKey {
    fn into_column_constraint(self) -> Option<ColumnConstraint> {
        Some(ColumnConstraint::PrimaryKey)
    }
}

/* Unique */

pub trait UniqueConstraint: IntoColumnConstraint {}

#[derive(Debug)]
pub struct IsUnique;

impl UniqueConstraint for NoConstraint {}
impl UniqueConstraint for IsUnique {}

impl IntoColumnConstraint for IsUnique {
    fn into_column_constraint(self) -> Option<ColumnConstraint> {
        Some(ColumnConstraint::Unique)
    }
}

/* Default */

pub trait DefaultConstraint: IntoColumnConstraint {}

#[derive(Debug)]
pub struct HasDefault(pub(crate) Expression);

impl DefaultConstraint for HasDefault {}
impl DefaultConstraint for NoConstraint {}

impl IntoColumnConstraint for HasDefault {
    fn into_column_constraint(self) -> Option<ColumnConstraint> {
        Some(ColumnConstraint::Default(self.0))
    }
}

/* References */

pub trait ReferencesConstraint: IntoColumnConstraint {}

#[derive(Debug)]
pub struct References(pub(crate) TableName, pub(crate) Column);

impl ReferencesConstraint for References {}
impl ReferencesConstraint for NoConstraint {}

impl IntoColumnConstraint for References {
    fn into_column_constraint(self) -> Option<ColumnConstraint> {
        Some(ColumnConstraint::References(self.0, self.1))
    }
}

/* Check */

pub trait CheckConstraint: IntoColumnConstraint {}

#[derive(Debug)]
pub struct Check(pub(crate) Condition);

impl CheckConstraint for Check {}
impl CheckConstraint for NoConstraint {}

impl IntoColumnConstraint for Check {
    fn into_column_constraint(self) -> Option<ColumnConstraint> {
        Some(ColumnConstraint::Check(self.0))
    }
}
