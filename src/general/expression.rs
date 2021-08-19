use std::{
    fmt::{self, Display, Formatter},
    iter::{Copied, Map},
    slice,
};

use crate::tools::{IntoArrayOfSameType, IntoIteratorOfSameType};

use super::Column;

#[derive(Debug, Eq, PartialEq)]
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

impl From<f32> for Expression {
    fn from(value: f32) -> Self {
        Expression(value.to_string())
    }
}

impl From<u32> for Expression {
    fn from(value: u32) -> Self {
        Expression(value.to_string())
    }
}

impl From<Column> for Expression {
    fn from(value: Column) -> Self {
        Expression(value.to_string())
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl IntoArrayOfSameType<Expression, 1> for &str {
    fn into_array(self) -> [Expression; 1] {
        [Expression(self.to_owned())]
    }
}

impl IntoArrayOfSameType<Expression, 1> for String {
    fn into_array(self) -> [Expression; 1] {
        [Expression(self)]
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
