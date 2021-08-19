use std::fmt::{self, Display, Formatter};

use crate::tools::IntoArrayOfSameType;

#[derive(Debug, Eq, PartialEq)]
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

impl IntoArrayOfSameType<Column, 1> for &str {
    fn into_array(self) -> [Column; 1] {
        [Column(self.to_owned())]
    }
}

impl IntoArrayOfSameType<Column, 1> for String {
    fn into_array(self) -> [Column; 1] {
        [Column(self)]
    }
}
