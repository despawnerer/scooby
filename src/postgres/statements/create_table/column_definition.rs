use std::fmt::{self, Display, Formatter};

use crate::postgres::general::{Column, Expression, TableName, Condition};
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

impl<N, P, U, D, R, C> From<ColumnDefinitionBuilder<N, P, U, D, R, C>> for ColumnDefinition
where
    N: NullabilityConstraint,
    P: PrimaryKeyConstraint,
    U: UniqueConstraint,
    D: DefaultConstraint,
    R: ReferencesConstraint,
    C: CheckConstraint,
{
    fn from(builder: ColumnDefinitionBuilder<N, P, U, D, R, C>) -> Self {
        let mut constraints = Vec::new();

        if let Some(constraint) = builder.nullability.into_column_constraint() {
            constraints.push(constraint);
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
            constraints.push(constraint);
        }

        if let Some(constraint) = builder.check.into_column_constraint() {
            constraints.push(constraint);
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
    C = NoConstraint,
> where
    N: NullabilityConstraint,
    P: PrimaryKeyConstraint,
    U: UniqueConstraint,
    D: DefaultConstraint,
    R: ReferencesConstraint,
    C: CheckConstraint,
{
    name: String,
    type_: String,
    nullability: N,
    primary_key: P,
    unique: U,
    default: D,
    references: R,
    check: C,
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
            check: NoConstraint,
        }
    }
}

impl<P, U, D, R, C> ColumnDefinitionBuilder<NoConstraint, P, U, D, R, C>
where
    P: PrimaryKeyConstraint,
    U: UniqueConstraint,
    D: DefaultConstraint,
    R: ReferencesConstraint,
    C: CheckConstraint,
{
    pub fn null(self) -> ColumnDefinitionBuilder<IsNull, P, U, D, R, C> {
        ColumnDefinitionBuilder {
            name: self.name,
            type_: self.type_,
            nullability: IsNull,
            primary_key: self.primary_key,
            unique: self.unique,
            default: self.default,
            references: self.references,
            check: self.check,
        }
    }

    pub fn not_null(self) -> ColumnDefinitionBuilder<IsNotNull, P, U, D, R, C> {
        ColumnDefinitionBuilder {
            name: self.name,
            type_: self.type_,
            nullability: IsNotNull,
            primary_key: self.primary_key,
            unique: self.unique,
            default: self.default,
            references: self.references,
                        check: self.check,
        }
    }
}

impl<N, U, D, R, C> ColumnDefinitionBuilder<N, NoConstraint, U, D, R, C>
where
    N: NullabilityConstraint,
    U: UniqueConstraint,
    D: DefaultConstraint,
    R: ReferencesConstraint,
    C: CheckConstraint,
{
    pub fn primary_key(self) -> ColumnDefinitionBuilder<N, IsPrimaryKey, U, D, R, C> {
        ColumnDefinitionBuilder {
            name: self.name,
            type_: self.type_,
            nullability: self.nullability,
            primary_key: IsPrimaryKey,
            unique: self.unique,
            default: self.default,
            references: self.references,
                        check: self.check,
        }
    }
}

impl<N, P, D, R, C> ColumnDefinitionBuilder<N, P, NoConstraint, D, R, C>
where
    N: NullabilityConstraint,
    P: PrimaryKeyConstraint,
    D: DefaultConstraint,
    R: ReferencesConstraint,
    C: CheckConstraint
{
    pub fn unique(self) -> ColumnDefinitionBuilder<N, P, IsUnique, D, R, C> {
        ColumnDefinitionBuilder {
            name: self.name,
            type_: self.type_,
            nullability: self.nullability,
            primary_key: self.primary_key,
            unique: IsUnique,
            default: self.default,
            references: self.references,
                        check: self.check,
        }
    }
}

impl<N, P, U, R, C> ColumnDefinitionBuilder<N, P, U, NoConstraint, R, C>
where
    N: NullabilityConstraint,
    P: PrimaryKeyConstraint,
    U: UniqueConstraint,
    R: ReferencesConstraint,
    C: CheckConstraint
{
    pub fn default(
        self,
        expr: impl Into<Expression>,
    ) -> ColumnDefinitionBuilder<N, P, U, HasDefault, R, C> {
        ColumnDefinitionBuilder {
            name: self.name,
            type_: self.type_,
            nullability: self.nullability,
            primary_key: self.primary_key,
            unique: self.unique,
            default: HasDefault(expr.into()),
            references: self.references,
                        check: self.check,
        }
    }
}

impl<N, P, U, D, C> ColumnDefinitionBuilder<N, P, U, D, NoConstraint, C>
where
    N: NullabilityConstraint,
    P: PrimaryKeyConstraint,
    U: UniqueConstraint,
    D: DefaultConstraint,
    C: CheckConstraint
{
    pub fn references(
        self,
        table_name: impl Into<TableName>,
        column: impl Into<Column>,
    ) -> ColumnDefinitionBuilder<N, P, U, D, References, C> {
        ColumnDefinitionBuilder {
            name: self.name,
            type_: self.type_,
            nullability: self.nullability,
            primary_key: self.primary_key,
            unique: self.unique,
            default: self.default,
            references: References(table_name.into(), column.into()),
                        check: self.check,
        }
    }
}


impl<N, P, U, D, R> ColumnDefinitionBuilder<N, P, U, D, R, NoConstraint>
where
    N: NullabilityConstraint,
    P: PrimaryKeyConstraint,
    U: UniqueConstraint,
    D: DefaultConstraint,
    R: ReferencesConstraint,
{
    pub fn check(
        self,
        cond: impl Into<Condition>
    ) -> ColumnDefinitionBuilder<N, P, U, D, R, Check> {
        ColumnDefinitionBuilder {
            name: self.name,
            type_: self.type_,
            nullability: self.nullability,
            primary_key: self.primary_key,
            unique: self.unique,
            default: self.default,
            references: self.references,
                        check: Check(cond.into()),
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
    fn check(self, cond: impl Into<Condition>) -> ColumnDefinitionBuilder<NoConstraint, NoConstraint, NoConstraint, NoConstraint, NoConstraint, Check>;
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

    fn check(
        self,
        expr: impl Into<Condition>,
    ) -> ColumnDefinitionBuilder<NoConstraint, NoConstraint, NoConstraint, NoConstraint, NoConstraint, Check> {
        ColumnDefinitionBuilder::from(self).check(expr)
    }
}
