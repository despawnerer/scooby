use std::fmt::{self, Display, Formatter};

/// `OFFSET` expression and things that can be converted into one
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Offset(String);

impl From<&str> for Offset {
    fn from(value: &str) -> Self {
        Offset(value.to_owned())
    }
}

impl From<String> for Offset {
    fn from(value: String) -> Self {
        Offset(value)
    }
}

impl From<usize> for Offset {
    fn from(value: usize) -> Self {
        Offset(value.to_string())
    }
}

impl Display for Offset {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
