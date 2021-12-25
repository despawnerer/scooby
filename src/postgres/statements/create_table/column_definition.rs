use std::fmt::{self, Display, Formatter};

use crate::postgres::general::Expression;
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

impl<N, P, U, D> From<ColumnDefinitionBuilder<N, P, U, D>> for ColumnDefinition
where
    N: NullabilityConstraint,
    P: PrimaryKeyConstraint,
    U: UniqueConstraint,
    D: DefaultConstraint,
{
    fn from(builder: ColumnDefinitionBuilder<N, P, U, D>) -> Self {
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
> where
    N: NullabilityConstraint,
    P: PrimaryKeyConstraint,
    U: UniqueConstraint,
    D: DefaultConstraint,
{
    name: String,
    type_: String,
    nullability: N,
    primary_key: P,
    unique: U,
    default: D,
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
        }
    }
}

impl<P, U, D> ColumnDefinitionBuilder<NoConstraint, P, U, D>
where
    P: PrimaryKeyConstraint,
    U: UniqueConstraint,
    D: DefaultConstraint,
{
    pub fn null(self) -> ColumnDefinitionBuilder<IsNull, P, U, D> {
        ColumnDefinitionBuilder {
            name: self.name,
            type_: self.type_,
            nullability: IsNull,
            primary_key: self.primary_key,
            unique: self.unique,
            default: self.default,
        }
    }

    pub fn not_null(self) -> ColumnDefinitionBuilder<IsNotNull, P, U, D> {
        ColumnDefinitionBuilder {
            name: self.name,
            type_: self.type_,
            nullability: IsNotNull,
            primary_key: self.primary_key,
            unique: self.unique,
            default: self.default,
        }
    }
}

impl<N, U, D> ColumnDefinitionBuilder<N, NoConstraint, U, D>
where
    N: NullabilityConstraint,
    U: UniqueConstraint,
    D: DefaultConstraint,
{
    pub fn primary_key(self) -> ColumnDefinitionBuilder<N, IsPrimaryKey, U, D> {
        ColumnDefinitionBuilder {
            name: self.name,
            type_: self.type_,
            nullability: self.nullability,
            primary_key: IsPrimaryKey,
            unique: self.unique,
            default: self.default,
        }
    }
}

impl<N, P, D> ColumnDefinitionBuilder<N, P, NoConstraint, D>
where
    N: NullabilityConstraint,
    P: PrimaryKeyConstraint,
    D: DefaultConstraint,
{
    pub fn unique(self) -> ColumnDefinitionBuilder<N, P, IsUnique, D> {
        ColumnDefinitionBuilder {
            name: self.name,
            type_: self.type_,
            nullability: self.nullability,
            primary_key: self.primary_key,
            unique: IsUnique,
            default: self.default,
        }
    }
}

impl<N, P, U> ColumnDefinitionBuilder<N, P, U, NoConstraint>
where
    N: NullabilityConstraint,
    P: PrimaryKeyConstraint,
    U: UniqueConstraint,
{
    pub fn default(
        self,
        expr: impl Into<Expression>,
    ) -> ColumnDefinitionBuilder<N, P, U, HasDefault> {
        ColumnDefinitionBuilder {
            name: self.name,
            type_: self.type_,
            nullability: self.nullability,
            primary_key: self.primary_key,
            unique: self.unique,
            default: HasDefault(expr.into()),
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
}
