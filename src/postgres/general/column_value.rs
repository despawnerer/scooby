use std::fmt::{self, Display, Formatter};

use super::{Column, Expression};

#[derive(Debug, Clone)]
pub struct ColumnValuePair {
    column: Column,
    expression: Expression,
}

impl Display for ColumnValuePair {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.column, self.expression)
    }
}

impl<T: Into<Column>, U: Into<Expression>> From<(T, U)> for ColumnValuePair {
    fn from(value: (T, U)) -> Self {
        ColumnValuePair {
            column: value.0.into(),
            expression: value.1.into(),
        }
    }
}
