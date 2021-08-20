use std::fmt::{self, Display, Formatter};
use std::iter::{once, Once};

use crate::postgres::Select;
use crate::tools::IntoIteratorOfSameType;

pub struct Alias {
    original: String,
    alias: String,
}

impl Display for Alias {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} AS {}", self.original, self.alias)
    }
}

impl From<Alias> for String {
    fn from(alias: Alias) -> Self {
        alias.to_string()
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

impl<T> IntoIteratorOfSameType<T> for Alias
where
    T: From<Alias>,
{
    type Iterator = Once<T>;

    fn into_some_iter(self) -> Self::Iterator {
        once(self.into())
    }
}
