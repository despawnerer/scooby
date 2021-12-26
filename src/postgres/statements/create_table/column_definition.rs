use std::fmt::{self, Display, Formatter};

use crate::postgres::general::{Column, Expression, TableName};
use crate::tools::joined;

use super::column_constraints::*;

#[derive(Debug, Clone)]
pub struct ColumnDefinition {
    name: String,
    type_: String,
    constraints: Vec<ColumnConstraint>,
}

impl Display for ColumnDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.type_)?;

        if !self.constraints.is_empty() {
            write!(f, " {}", joined(&self.constraints, " "))?
        }

        Ok(())
    }
}

impl<N, P, U, D, R> From<ColumnDefinitionBuilder<N, P, U, D, R>> for ColumnDefinition
where
    N: NullabilityConstraint,
    P: PrimaryKeyConstraint,
    U: UniqueConstraint,
    D: DefaultConstraint,
    R: ReferencesConstraint,
{
    fn from(builder: ColumnDefinitionBuilder<N, P, U, D, R>) -> Self {
        let mut constraints = Vec::new();

        if let Some(constraint) = builder.nullability.into_column_constraint() {
            constraints.push(constraint)
        }

        if let Some(constraint) = builder.primary_key.into_column_constraint() {
            constraints.push(constraint);
        }

        if let Some(constraint) = builder.unique.into_column_constraint() {
            constraints.push(constraint);
        }

        if let Some(constraint) = builder.default.into_column_constraint() {
            constraints.push(constraint);
        }

        if let Some(constraint) = builder.references.into_column_constraint() {
            constraints.push(constraint)
        }

        ColumnDefinition {
            name: builder.name,
            type_: builder.type_,
            constraints,
        }
    }
}

impl<T, U> From<(T, U)> for ColumnDefinition
where
    T: Into<String>,
    U: Into<String>,
{
    fn from((name, type_): (T, U)) -> Self {
        ColumnDefinition {
            name: name.into(),
            type_: type_.into(),
            constraints: Vec::new(),
        }
    }
}

/* A super-flexible and type-safe builder of column definitions */

#[derive(Debug)]
pub struct ColumnDefinitionBuilder<
    N = NoConstraint,
    P = NoConstraint,
    U = NoConstraint,
    D = NoConstraint,
    R = NoConstraint,
> where
    N: NullabilityConstraint,
    P: PrimaryKeyConstraint,
    U: UniqueConstraint,
    D: DefaultConstraint,
    R: ReferencesConstraint,
{
    name: String,
    type_: String,
    nullability: N,
    primary_key: P,
    unique: U,
    default: D,
    references: R,
}

impl ColumnDefinitionBuilder {
    fn new(name: String, type_: String) -> ColumnDefinitionBuilder {
        ColumnDefinitionBuilder {
            name,
            type_,
            nullability: NoConstraint,
            primary_key: NoConstraint,
            unique: NoConstraint,
            default: NoConstraint,
            references: NoConstraint,
        }
    }
}

impl<P, U, D, R> ColumnDefinitionBuilder<NoConstraint, P, U, D, R>
where
    P: PrimaryKeyConstraint,
    U: UniqueConstraint,
    D: DefaultConstraint,
    R: ReferencesConstraint,
{
    pub fn null(self) -> ColumnDefinitionBuilder<IsNull, P, U, D, R> {
        ColumnDefinitionBuilder {
            name: self.name,
            type_: self.type_,
            nullability: IsNull,
            primary_key: self.primary_key,
            unique: self.unique,
            default: self.default,
            references: self.references,
        }
    }

    pub fn not_null(self) -> ColumnDefinitionBuilder<IsNotNull, P, U, D, R> {
        ColumnDefinitionBuilder {
            name: self.name,
            type_: self.type_,
            nullability: IsNotNull,
            primary_key: self.primary_key,
            unique: self.unique,
            default: self.default,
            references: self.references,
        }
    }
}

impl<N, U, D, R> ColumnDefinitionBuilder<N, NoConstraint, U, D, R>
where
    N: NullabilityConstraint,
    U: UniqueConstraint,
    D: DefaultConstraint,
    R: ReferencesConstraint,
{
    pub fn primary_key(self) -> ColumnDefinitionBuilder<N, IsPrimaryKey, U, D, R> {
        ColumnDefinitionBuilder {
            name: self.name,
            type_: self.type_,
            nullability: self.nullability,
            primary_key: IsPrimaryKey,
            unique: self.unique,
            default: self.default,
            references: self.references,
        }
    }
}

impl<N, P, D, R> ColumnDefinitionBuilder<N, P, NoConstraint, D, R>
where
    N: NullabilityConstraint,
    P: PrimaryKeyConstraint,
    D: DefaultConstraint,
    R: ReferencesConstraint,
{
    pub fn unique(self) -> ColumnDefinitionBuilder<N, P, IsUnique, D, R> {
        ColumnDefinitionBuilder {
            name: self.name,
            type_: self.type_,
            nullability: self.nullability,
            primary_key: self.primary_key,
            unique: IsUnique,
            default: self.default,
            references: self.references,
        }
    }
}

impl<N, P, U, R> ColumnDefinitionBuilder<N, P, U, NoConstraint, R>
where
    N: NullabilityConstraint,
    P: PrimaryKeyConstraint,
    U: UniqueConstraint,
    R: ReferencesConstraint,
{
    pub fn default(
        self,
        expr: impl Into<Expression>,
    ) -> ColumnDefinitionBuilder<N, P, U, HasDefault, R> {
        ColumnDefinitionBuilder {
            name: self.name,
            type_: self.type_,
            nullability: self.nullability,
            primary_key: self.primary_key,
            unique: self.unique,
            default: HasDefault(expr.into()),
            references: self.references,
        }
    }
}

impl<N, P, U, D> ColumnDefinitionBuilder<N, P, U, D, NoConstraint>
where
    N: NullabilityConstraint,
    P: PrimaryKeyConstraint,
    U: UniqueConstraint,
    D: DefaultConstraint,
{
    pub fn references(
        self,
        table_name: impl Into<TableName>,
        column: impl Into<Column>,
    ) -> ColumnDefinitionBuilder<N, P, U, D, References> {
        ColumnDefinitionBuilder {
            name: self.name,
            type_: self.type_,
            nullability: self.nullability,
            primary_key: self.primary_key,
            unique: self.unique,
            default: self.default,
            references: References(table_name.into(), column.into()),
        }
    }
}

impl<T, U> From<(T, U)> for ColumnDefinitionBuilder
where
    T: Into<String>,
    U: Into<String>,
{
    fn from((name, type_): (T, U)) -> Self {
        ColumnDefinitionBuilder::new(name.into(), type_.into())
    }
}

pub trait ColumnDefinitionable: Into<ColumnDefinitionBuilder> {
    fn null(self) -> ColumnDefinitionBuilder<IsNull>;
    fn not_null(self) -> ColumnDefinitionBuilder<IsNotNull>;
    fn primary_key(self) -> ColumnDefinitionBuilder<NoConstraint, IsPrimaryKey>;
    fn unique(self) -> ColumnDefinitionBuilder<NoConstraint, NoConstraint, IsUnique>;
    fn default(
        self,
        expr: impl Into<Expression>,
    ) -> ColumnDefinitionBuilder<NoConstraint, NoConstraint, NoConstraint, HasDefault>;
    fn references(
        self,
        table_name: impl Into<TableName>,
        column: impl Into<Column>,
    ) -> ColumnDefinitionBuilder<NoConstraint, NoConstraint, NoConstraint, NoConstraint, References>;
}

impl<T, U> ColumnDefinitionable for (T, U)
where
    T: Into<String>,
    U: Into<String>,
{
    fn null(self) -> ColumnDefinitionBuilder<IsNull> {
        ColumnDefinitionBuilder::from(self).null()
    }

    fn not_null(self) -> ColumnDefinitionBuilder<IsNotNull> {
        ColumnDefinitionBuilder::from(self).not_null()
    }

    fn primary_key(self) -> ColumnDefinitionBuilder<NoConstraint, IsPrimaryKey> {
        ColumnDefinitionBuilder::from(self).primary_key()
    }

    fn unique(self) -> ColumnDefinitionBuilder<NoConstraint, NoConstraint, IsUnique> {
        ColumnDefinitionBuilder::from(self).unique()
    }

    fn default(
        self,
        expr: impl Into<Expression>,
    ) -> ColumnDefinitionBuilder<NoConstraint, NoConstraint, NoConstraint, HasDefault> {
        ColumnDefinitionBuilder::from(self).default(expr)
    }

    fn references(
        self,
        table_name: impl Into<TableName>,
        column: impl Into<Column>,
    ) -> ColumnDefinitionBuilder<NoConstraint, NoConstraint, NoConstraint, NoConstraint, References>
    {
        ColumnDefinitionBuilder::from(self).references(table_name, column)
    }
}
