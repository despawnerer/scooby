use std::fmt::{self, Display, Formatter};

/// `LIMIT` expression and things that can be converted into one
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Limit(String);

impl From<&str> for Limit {
    fn from(value: &str) -> Self {
        Limit(value.to_owned())
    }
}

impl From<String> for Limit {
    fn from(value: String) -> Self {
        Limit(value)
    }
}

impl From<usize> for Limit {
    fn from(value: usize) -> Self {
        Limit(value.to_string())
    }
}

impl Display for Limit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
