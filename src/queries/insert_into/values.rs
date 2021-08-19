use std::fmt::{self, Display, Formatter};

use itertools::Itertools;

use crate::general::Expression;

#[derive(Debug)]
pub enum Values<const N: usize> {
    Default,
    List(Vec<[Expression; N]>),
}

impl<const N: usize> Display for Values<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Values::Default => write!(f, "DEFAULT VALUES"),
            Values::List(rows) if rows.len() == 0 => write!(f, "VALUES ()"),
            Values::List(rows) => write!(
                f,
                "VALUES {}",
                rows.iter()
                    .map(|cols| format!("({})", cols.iter().join(", ")))
                    .join(", ")
            ),
        }
    }
}
