use std::fmt::{self, Display, Formatter};
use std::iter::{once, Once};

use itertools::Itertools;

use crate::tools::IntoIteratorOfSameType;
use crate::{Alias, Select};
use crate::general::TableName;

#[derive(Debug)]
pub struct FromItem {
    table_name: TableName,
    joins: Vec<Join>,
}

impl FromItem {
    fn new(table_name: impl Into<TableName>) -> FromItem {
        FromItem {
            table_name: table_name.into(),
            joins: Vec::new(),
        }
    }
}

impl Display for FromItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.table_name)?;

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

    pub fn using(self, columns: impl IntoIteratorOfSameType<String>) -> FromItem {
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
    fn join(self, to: impl Into<FromItem>) -> JoinBuilder;
    fn inner_join(self, to: impl Into<FromItem>) -> JoinBuilder;
    fn left_join(self, to: impl Into<FromItem>) -> JoinBuilder;
    fn left_outer_join(self, to: impl Into<FromItem>) -> JoinBuilder;
    fn right_join(self, to: impl Into<FromItem>) -> JoinBuilder;
    fn right_outer_join(self, to: impl Into<FromItem>) -> JoinBuilder;
    fn full_join(self, to: impl Into<FromItem>) -> JoinBuilder;
    fn full_outer_join(self, to: impl Into<FromItem>) -> JoinBuilder;
    fn cross_join(self, to: impl Into<FromItem>) -> FromItem;
}

impl<T> Joinable for T
where
    T: Into<FromItem>,
{
    fn join(self, to: impl Into<FromItem>) -> JoinBuilder {
        JoinBuilder {
            from: self.into(),
            to: to.into(),
            type_: JoinType::Unspecified,
        }
    }

    fn inner_join(self, to: impl Into<FromItem>) -> JoinBuilder {
        JoinBuilder {
            from: self.into(),
            to: to.into(),
            type_: JoinType::Inner,
        }
    }

    fn left_join(self, to: impl Into<FromItem>) -> JoinBuilder {
        JoinBuilder {
            from: self.into(),
            to: to.into(),
            type_: JoinType::Left,
        }
    }

    fn left_outer_join(self, to: impl Into<FromItem>) -> JoinBuilder {
        JoinBuilder {
            from: self.into(),
            to: to.into(),
            type_: JoinType::LeftOuter,
        }
    }

    fn right_join(self, to: impl Into<FromItem>) -> JoinBuilder {
        JoinBuilder {
            from: self.into(),
            to: to.into(),
            type_: JoinType::Right,
        }
    }

    fn right_outer_join(self, to: impl Into<FromItem>) -> JoinBuilder {
        JoinBuilder {
            from: self.into(),
            to: to.into(),
            type_: JoinType::RightOuter,
        }
    }

    fn full_join(self, to: impl Into<FromItem>) -> JoinBuilder {
        JoinBuilder {
            from: self.into(),
            to: to.into(),
            type_: JoinType::Full,
        }
    }

    fn full_outer_join(self, to: impl Into<FromItem>) -> JoinBuilder {
        JoinBuilder {
            from: self.into(),
            to: to.into(),
            type_: JoinType::FullOuter,
        }
    }

    fn cross_join(self, to: impl Into<FromItem>) -> FromItem {
        let mut from = self.into();

        from.joins.push(Join {
            type_: JoinType::Cross,
            to: to.into(),
            condition: None,
        });

        from
    }
}

/* Conversions */

impl From<String> for FromItem {
    fn from(other: String) -> FromItem {
        FromItem::new(other)
    }
}

impl From<&str> for FromItem {
    fn from(other: &str) -> FromItem {
        FromItem::new(other.to_string())
    }
}

impl From<Alias> for FromItem {
    fn from(other: Alias) -> FromItem {
        FromItem::new(other.to_string())
    }
}

impl From<Select> for FromItem {
    fn from(other: Select) -> FromItem {
        FromItem::new(format!("({})", other))
    }
}

impl IntoIteratorOfSameType<FromItem> for Select {
    type Iterator = Once<FromItem>;

    fn into_some_iter(self) -> Self::Iterator {
        once(self.into())
    }
}

impl<T> IntoIteratorOfSameType<T> for FromItem
where
    T: From<FromItem>,
{
    type Iterator = Once<T>;

    fn into_some_iter(self) -> Self::Iterator {
        once(self.into())
    }
}
