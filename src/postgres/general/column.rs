use std::fmt::{self, Display, Formatter};

use crate::tools::IntoNonZeroArray;

/// Column name and things that can be converted into one
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Column(String);

impl From<&str> for Column {
    fn from(s: &str) -> Self {
        Column(s.to_owned())
    }
}

impl From<String> for Column {
    fn from(s: String) -> Self {
        Column(s)
    }
}

impl Display for Column {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl IntoNonZeroArray<Column, 1> for &str {
    fn into_non_zero_array(self) -> [Column; 1] {
        [Column(self.to_owned())]
    }
}

impl IntoNonZeroArray<Column, 1> for String {
    fn into_non_zero_array(self) -> [Column; 1] {
        [Column(self)]
    }
}
