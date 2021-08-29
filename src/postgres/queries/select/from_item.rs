use std::fmt::{self, Display, Formatter};
use std::iter::{once, Once};

use itertools::Itertools;

use crate::postgres::general::{Alias, TableName};
use crate::postgres::queries::Select;
use crate::tools::IntoIteratorOfSameType;

use super::join::Join;

#[derive(Debug, Clone)]
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

    pub fn has_joins(&self) -> bool {
        self.joins.len() > 0
    }

    pub fn add_join(&mut self, join: Join) {
        self.joins.push(join);
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
