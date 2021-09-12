use std::fmt::{self, Display, Formatter};
use std::iter::{once, Once};

use crate::postgres::general::{Alias, TableName};
use crate::tools::{joined, IntoIteratorOfSameType};

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
        !self.joins.is_empty()
    }

    pub fn add_join(&mut self, join: Join) {
        self.joins.push(join);
    }
}

impl Display for FromItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.table_name)?;

        if self.has_joins() {
            write!(f, " {}", joined(&self.joins, " "))?;
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

impl<T> IntoIteratorOfSameType<T> for FromItem
where
    T: From<FromItem>,
{
    type Iterator = Once<T>;

    fn into_some_iter(self) -> Self::Iterator {
        once(self.into())
    }
}
