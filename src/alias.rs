use std::fmt::{self, Display, Formatter};

use crate::select::Select;

pub struct Alias {
    original: String,
    alias: String,
}

impl Display for Alias {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} AS {}", self.original, self.alias)
    }
}

pub trait Aliasable {
    fn as_(self, alias: &str) -> Alias;
}

impl<T> Aliasable for T
where
    T: AsRef<str>,
{
    fn as_(self, alias: &str) -> Alias {
        Alias {
            original: self.as_ref().to_string(),
            alias: alias.to_string(),
        }
    }
}

impl Aliasable for Select {
    fn as_(self, alias: &str) -> Alias {
        Alias {
            original: format!("({})", self),
            alias: alias.to_string(),
        }
    }
}
