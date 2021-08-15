use std::iter::{once, Once};
use std::fmt::{self, Display, Formatter};

use crate::general::SortExpression;
use crate::tools::IntoSomeIterator;

#[derive(Debug)]
pub struct OrderBy {
    expression: SortExpression,
    direction: Option<Direction>,
    nulls: Option<Nulls>,
}

impl OrderBy {
    fn new(expression: SortExpression) -> Self {
        OrderBy {
            expression,
            direction: None,
            nulls: None,
        }
    }

    fn desc(mut self) -> Self {
        self.direction = Some(Direction::Desc);
        self
    }

    fn asc(mut self) -> Self {
        self.direction = Some(Direction::Asc);
        self
    }

    fn nulls_first(mut self) -> Self {
        self.nulls = Some(Nulls::First);
        self
    }

    fn nulls_last(mut self) -> Self {
        self.nulls = Some(Nulls::Last);
        self
    }
}

impl Display for OrderBy {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expression)?;

        if let Some(direction) = &self.direction {
            write!(f, " {}", direction)?;
        }

        if let Some(nulls) = &self.nulls {
            write!(f, " {}", nulls)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
enum Nulls {
    First,
    Last,
}

impl Display for Nulls {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Nulls::First => write!(f, "NULLS FIRST"),
            Nulls::Last => write!(f, "NULLS LAST"),
        }
    }
}

#[derive(Debug)]
enum Direction {
    Asc,
    Desc,
    // TODO: USING?
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Asc => write!(f, "ASC"),
            Direction::Desc => write!(f, "DESC"),
        }
    }
}

/* Convenience */

pub trait Orderable {
    fn desc(self) -> OrderBy;
    fn asc(self) -> OrderBy;
    fn nulls_first(self) -> OrderBy;
    fn nulls_last(self) -> OrderBy;
}

impl<T> Orderable for T
where
    T: Into<OrderBy>,
{
    fn desc(self) -> OrderBy {
        self.into().desc()
    }

    fn asc(self) -> OrderBy {
        self.into().asc()
    }

    fn nulls_first(self) -> OrderBy {
        self.into().nulls_first()
    }

    fn nulls_last(self) -> OrderBy {
        self.into().nulls_last()
    }
}

/* Conversions */

impl<T> From<T> for OrderBy where T: Into<SortExpression> {
    fn from(other: T) -> Self {
        OrderBy::new(other.into())
    }
}

impl<T> IntoSomeIterator<T> for OrderBy where T: From<OrderBy> {
    type Iterator = Once<T>;

    fn into_some_iter(self) -> Self::Iterator {
        once(self.into())
    }
}
