use std::fmt::{self, Display, Formatter};
use std::iter::{once, Chain, Once};

use itertools::Itertools;

use crate::tools::IntoSomeIterator;
use crate::{Alias, Select};

#[derive(Debug)]
pub struct FromItem {
    table: String,
    joins: Vec<Join>,
}

impl FromItem {
    fn new(table: String) -> FromItem {
        FromItem {
            table: table,
            joins: Vec::new(),
        }
    }
}

impl Display for FromItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.table)?;

        if self.joins.len() > 0 {
            write!(f, " {}", self.joins.iter().join(" "))? // ha, I'm joining joins, get it?
        }

        Ok(())
    }
}

/* Joins */

#[derive(Debug)]
enum JoinType {
    Unspecified,
    Inner,
    Left,
    LeftOuter,
    Right,
    RightOuter,
    Full,
    FullOuter,
    Cross,
}

impl Display for JoinType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                JoinType::Unspecified => "JOIN",
                JoinType::Inner => "INNER JOIN",
                JoinType::Left => "LEFT JOIN",
                JoinType::LeftOuter => "LEFT OUTER JOIN",
                JoinType::Right => "RIGHT JOIN",
                JoinType::RightOuter => "RIGHT OUTER JOIN",
                JoinType::Full => "FULL JOIN",
                JoinType::FullOuter => "FULL OUTER JOIN",
                JoinType::Cross => "CROSS JOIN",
            }
        )
    }
}

#[derive(Debug)]
enum JoinCondition {
    On(String),
    Using(Vec<String>),
}

impl Display for JoinCondition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            JoinCondition::On(expr) => write!(f, "ON {}", expr),
            JoinCondition::Using(columns) => write!(f, "USING ({})", columns.iter().join(", ")),
        }
    }
}

#[derive(Debug)]
struct Join {
    type_: JoinType,
    to: FromItem,
    condition: Option<JoinCondition>,
}

impl Display for Join {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.to.joins.len() > 0 {
            write!(f, "{} ({})", self.type_, self.to)?;
        } else {
            write!(f, "{} {}", self.type_, self.to)?;
        }

        if let Some(condition) = &self.condition {
            write!(f, " {}", condition)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct JoinBuilder {
    from: FromItem,
    to: FromItem,
    type_: JoinType,
}

impl JoinBuilder {
    pub fn on(self, condition: &str) -> FromItem {
        let mut from = self.from;

        from.joins.push(Join {
            type_: self.type_,
            to: self.to,
            condition: Some(JoinCondition::On(condition.to_string())),
        });

        from
    }

    pub fn using(self, columns: impl IntoSomeIterator<String>) -> FromItem {
        let mut from = self.from;

        from.joins.push(Join {
            type_: self.type_,
            to: self.to,
            condition: Some(JoinCondition::Using(columns.into_some_iter().collect())),
        });

        from
    }
}

/* Flexible join-building thing */

pub trait Joinable {
    fn join(self, to: impl ToFromItem) -> JoinBuilder;
    fn inner_join(self, to: impl ToFromItem) -> JoinBuilder;
    fn left_join(self, to: impl ToFromItem) -> JoinBuilder;
    fn left_outer_join(self, to: impl ToFromItem) -> JoinBuilder;
    fn right_join(self, to: impl ToFromItem) -> JoinBuilder;
    fn right_outer_join(self, to: impl ToFromItem) -> JoinBuilder;
    fn full_join(self, to: impl ToFromItem) -> JoinBuilder;
    fn full_outer_join(self, to: impl ToFromItem) -> JoinBuilder;
    fn cross_join(self, to: impl ToFromItem) -> FromItem;
}

impl<T> Joinable for T
where
    T: ToFromItem,
{
    fn join(self, to: impl ToFromItem) -> JoinBuilder {
        JoinBuilder {
            from: self.to_from_item(),
            to: to.to_from_item(),
            type_: JoinType::Unspecified,
        }
    }

    fn inner_join(self, to: impl ToFromItem) -> JoinBuilder {
        JoinBuilder {
            from: self.to_from_item(),
            to: to.to_from_item(),
            type_: JoinType::Inner,
        }
    }

    fn left_join(self, to: impl ToFromItem) -> JoinBuilder {
        JoinBuilder {
            from: self.to_from_item(),
            to: to.to_from_item(),
            type_: JoinType::Left,
        }
    }

    fn left_outer_join(self, to: impl ToFromItem) -> JoinBuilder {
        JoinBuilder {
            from: self.to_from_item(),
            to: to.to_from_item(),
            type_: JoinType::LeftOuter,
        }
    }

    fn right_join(self, to: impl ToFromItem) -> JoinBuilder {
        JoinBuilder {
            from: self.to_from_item(),
            to: to.to_from_item(),
            type_: JoinType::Right,
        }
    }

    fn right_outer_join(self, to: impl ToFromItem) -> JoinBuilder {
        JoinBuilder {
            from: self.to_from_item(),
            to: to.to_from_item(),
            type_: JoinType::RightOuter,
        }
    }

    fn full_join(self, to: impl ToFromItem) -> JoinBuilder {
        JoinBuilder {
            from: self.to_from_item(),
            to: to.to_from_item(),
            type_: JoinType::Full,
        }
    }

    fn full_outer_join(self, to: impl ToFromItem) -> JoinBuilder {
        JoinBuilder {
            from: self.to_from_item(),
            to: to.to_from_item(),
            type_: JoinType::FullOuter,
        }
    }

    fn cross_join(self, to: impl ToFromItem) -> FromItem {
        let mut from = self.to_from_item();

        from.joins.push(Join {
            type_: JoinType::Cross,
            to: to.to_from_item(),
            condition: None,
        });

        from
    }
}

/* Conversions */

pub trait ToFromItem {
    fn to_from_item(self) -> FromItem;
}

impl ToFromItem for String {
    fn to_from_item(self) -> FromItem {
        FromItem::new(self)
    }
}

impl ToFromItem for &str {
    fn to_from_item(self) -> FromItem {
        FromItem::new(self.to_string())
    }
}

impl ToFromItem for FromItem {
    fn to_from_item(self) -> FromItem {
        self
    }
}

impl ToFromItem for Alias {
    fn to_from_item(self) -> FromItem {
        FromItem::new(self.to_string())
    }
}

impl ToFromItem for Select {
    fn to_from_item(self) -> FromItem {
        FromItem::new(format!("({})", self))
    }
}

/* Iterator flexibility support */

impl<T> IntoSomeIterator<FromItem> for T
where
    T: ToFromItem,
{
    type Iterator = Once<FromItem>;

    fn into_some_iter(self) -> Self::Iterator {
        once(self.to_from_item())
    }
}

impl<A, B> IntoSomeIterator<FromItem> for (A, B)
where
    A: ToFromItem,
    B: ToFromItem,
{
    type Iterator = Chain<Once<FromItem>, Once<FromItem>>;

    fn into_some_iter(self) -> Self::Iterator {
        once(self.0.to_from_item()).chain(once(self.1.to_from_item()))
    }
}

impl<A, B, C> IntoSomeIterator<FromItem> for (A, B, C)
where
    A: ToFromItem,
    B: ToFromItem,
    C: ToFromItem,
{
    type Iterator = Chain<Chain<Once<FromItem>, Once<FromItem>>, Once<FromItem>>;

    fn into_some_iter(self) -> Self::Iterator {
        once(self.0.to_from_item())
            .chain(once(self.1.to_from_item()))
            .chain(once(self.2.to_from_item()))
    }
}
