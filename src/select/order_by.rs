use std::fmt::{self, Display, Formatter};
use std::iter::{once, Chain, Once};

use crate::select::Expression;
use crate::tools::IntoSomeIterator;

#[derive(Debug)]
pub struct OrderBy {
    expression: Expression,
    direction: Option<Direction>,
    nulls: Option<Nulls>,
}

impl OrderBy {
    fn new(expression: Expression) -> Self {
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

/* Conversions */

pub trait ToOrderBy {
    fn to_order_by(self) -> OrderBy;
}

impl ToOrderBy for String {
    fn to_order_by(self) -> OrderBy {
        OrderBy::new(self)
    }
}

impl ToOrderBy for &str {
    fn to_order_by(self) -> OrderBy {
        OrderBy::new(self.to_owned())
    }
}

impl ToOrderBy for OrderBy {
    fn to_order_by(self) -> OrderBy {
        self
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
    T: ToOrderBy,
{
    fn desc(self) -> OrderBy {
        self.to_order_by().desc()
    }

    fn asc(self) -> OrderBy {
        self.to_order_by().asc()
    }

    fn nulls_first(self) -> OrderBy {
        self.to_order_by().nulls_first()
    }

    fn nulls_last(self) -> OrderBy {
        self.to_order_by().nulls_last()
    }
}

/* Iterator-based flexibility */

impl<T> IntoSomeIterator<OrderBy> for T
where
    T: ToOrderBy,
{
    type Iterator = Once<OrderBy>;

    fn into_some_iter(self) -> Self::Iterator {
        once(self.to_order_by())
    }
}

impl<A, B> IntoSomeIterator<OrderBy> for (A, B)
where
    A: ToOrderBy,
    B: ToOrderBy,
{
    type Iterator = Chain<Once<OrderBy>, Once<OrderBy>>;

    fn into_some_iter(self) -> Self::Iterator {
        once(self.0.to_order_by()).chain(once(self.1.to_order_by()))
    }
}
impl<A, B, C> IntoSomeIterator<OrderBy> for (A, B, C)
where
    A: ToOrderBy,
    B: ToOrderBy,
    C: ToOrderBy,
{
    type Iterator = Chain<Chain<Once<OrderBy>, Once<OrderBy>>, Once<OrderBy>>;

    fn into_some_iter(self) -> Self::Iterator {
        once(self.0.to_order_by()).chain(once(self.1.to_order_by())).chain(once(self.2.to_order_by()))
    }
}

