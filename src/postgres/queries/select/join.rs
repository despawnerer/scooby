use std::fmt::{self, Display, Formatter};

use itertools::Itertools;

use crate::tools::IntoIteratorOfSameType;

use super::FromItem;

/* Complete representation of a join */

#[derive(Debug, Clone)]
pub struct Join {
    type_: JoinType,
    to: FromItem,
    condition: Option<JoinCondition>,
}

impl Display for Join {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.to.has_joins() {
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

        from.add_join(Join {
            type_: JoinType::Cross,
            to: to.into(),
            condition: None,
        });

        from
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

        from.add_join(Join {
            type_: self.type_,
            to: self.to,
            condition: Some(JoinCondition::On(condition.to_string())),
        });

        from
    }

    pub fn using(self, columns: impl IntoIteratorOfSameType<String>) -> FromItem {
        let mut from = self.from;

        from.add_join(Join {
            type_: self.type_,
            to: self.to,
            condition: Some(JoinCondition::Using(columns.into_some_iter().collect())),
        });

        from
    }
}
