use std::fmt::{self, Display, Formatter};

use crate::{postgres::general::Column, tools::joined};

#[derive(Debug, Clone)]
pub enum TableConstraint {
    Unique(Vec<Column>),
}

impl Display for TableConstraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unique(columns) => write!(f, "UNIQUE ({})", joined(columns, ", ")),
        }
    }
}
