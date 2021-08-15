use std::fmt::{self, Display, Formatter};

use itertools::Itertools;

use crate::general::{Column, Expression};
use crate::tools::IntoArray;

pub fn insert_into<const N: usize>(table_name: &str, columns: impl IntoArray<Column, N>) -> InsertInto<N> {
    InsertInto {
        table_name: table_name.to_string(),
        columns: columns.into_array(),
        values: Values::List(Vec::new()),
    }
}

#[derive(Debug)]
pub struct InsertInto<const N: usize> {
    table_name: String,
    columns: [Column; N],
    values: Values<N>,
}

impl<const N: usize> InsertInto<N> {
    pub fn default_values(mut self) -> Self {
        self.values = Values::Default;
        self
    }

    pub fn values<T: IntoArray<Expression, N>>(mut self, values: impl IntoIterator<Item = T>) -> Self {
        let iter = values.into_iter().map(IntoArray::into_array);

        match self.values {
            Values::Default => self.values = Values::List(iter.collect()),
            Values::List(ref mut vec) => vec.extend(iter),
        }

        self
    }
}

impl<const N: usize> Display for InsertInto<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "INSERT INTO {} ({}) {}", self.table_name, self.columns.iter().join(", "), self.values)?;

        Ok(())
    }
}

#[derive(Debug)]
enum Values<const N: usize> {
    Default,
    List(Vec<[Expression; N]>),
}

impl<const N: usize> Display for Values<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Values::Default => write!(f, "DEFAULT VALUES"),
            Values::List(rows) if rows.len() == 0 => write!(f, "VALUES ()"),
            Values::List(rows) => write!(f, "VALUES {}", rows.iter().map(|cols| format!("({})", cols.iter().join(", "))).join(", ")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::insert_into;

    #[test]
    fn no_values() {
        let sql = insert_into("Dummy", ()).to_string();
        assert_eq!(sql, "INSERT INTO Dummy () VALUES ()");
    }

    #[test]
    fn default_values() {
        let sql = insert_into("Dummy", ()).default_values().to_string();
        assert_eq!(sql, "INSERT INTO Dummy () DEFAULT VALUES");
    }

    #[test]
    fn single_column() {
        let sql = insert_into("Dummy", "col1").values(["a"]).to_string();
        assert_eq!(sql, "INSERT INTO Dummy (col1) VALUES (a)");
    }

    #[test]
    fn values() {
        let sql = insert_into("Dummy", ("col1", "col2")).values([("a", "b")]).to_string();
        assert_eq!(sql, "INSERT INTO Dummy (col1, col2) VALUES (a, b)");
    }

    #[test]
    fn values_many() {
        let sql = insert_into("Dummy", ("col1", "col2")).values([("a", "b"), ("c", "d")]).to_string();
        assert_eq!(sql, "INSERT INTO Dummy (col1, col2) VALUES (a, b), (c, d)");
    }
}
