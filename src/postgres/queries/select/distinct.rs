use std::fmt::{self, Display, Formatter};

use itertools::Itertools;

use crate::postgres::general::Expression;

#[derive(Debug, Clone)]
pub enum Distinct {
    All,
    Distinct,
    DistinctOn(Vec<Expression>),
}

impl Display for Distinct {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Distinct::All => write!(f, "ALL"),
            Distinct::Distinct => write!(f, "DISTINCT"),
            Distinct::DistinctOn(expressions) => {
                write!(f, "DISTINCT ON ({})", expressions.iter().join(", "))
            }
        }
    }
}
