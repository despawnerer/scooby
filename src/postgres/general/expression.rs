use std::{
    fmt::{self, Display, Formatter},
    iter::{Copied, Map},
    slice,
};

use crate::tools::{transform_array, IntoIteratorOfSameType, IntoNonZeroArray};

use super::{Alias, Column};

/// Expression and things that can be converted into one
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Expression(String);

impl From<&str> for Expression {
    fn from(value: &str) -> Self {
        Expression(value.to_owned())
    }
}

impl From<String> for Expression {
    fn from(value: String) -> Self {
        Expression(value)
    }
}

impl From<Column> for Expression {
    fn from(value: Column) -> Self {
        Expression(value.to_string())
    }
}

impl From<Alias> for Expression {
    fn from(value: Alias) -> Self {
        Expression(value.to_string())
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl IntoNonZeroArray<Expression, 1> for &str {
    fn into_non_zero_array(self) -> [Expression; 1] {
        [Expression(self.to_owned())]
    }
}

impl IntoNonZeroArray<Expression, 1> for String {
    fn into_non_zero_array(self) -> [Expression; 1] {
        [Expression(self)]
    }
}

impl<const N: usize> IntoNonZeroArray<Expression, N> for [String; N] {
    fn into_non_zero_array(self) -> [Expression; N] {
        transform_array(self, Expression)
    }
}

impl<'a, T> IntoIteratorOfSameType<Expression> for &'a T
where
    T: AsRef<[&'a str]>,
{
    // Jesus fucking Christ almighty, there is no God.
    type Iterator = Map<Copied<slice::Iter<'a, &'a str>>, fn(&'a str) -> Expression>;

    fn into_some_iter(self) -> Self::Iterator {
        self.as_ref().iter().copied().map(Expression::from)
    }
}
