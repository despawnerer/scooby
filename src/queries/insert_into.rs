mod values;

use std::fmt::{self, Display, Formatter};

use itertools::Itertools;

use crate::general::{Column, Expression, OutputExpression, TableName};
use crate::tools::{IntoArrayOfSameType, IntoIteratorOfSameType};

pub use values::Values;

pub fn insert_into<const N: usize>(
    table_name: impl Into<TableName>,
    columns: impl IntoArrayOfSameType<Column, N>,
) -> InsertInto<N> {
    InsertInto {
        table_name: table_name.into(),
        columns: columns.into_array(),
        values: Values::List(Vec::new()),
        returning: Vec::new(),
    }
}

#[derive(Debug)]
pub struct InsertInto<const N: usize> {
    table_name: TableName,
    columns: [Column; N],
    values: Values<N>,
    returning: Vec<OutputExpression>,
}

impl<const N: usize> InsertInto<N> {
    pub fn default_values(mut self) -> Self {
        self.values = Values::Default;
        self
    }

    pub fn values<T: IntoArrayOfSameType<Expression, N>>(
        mut self,
        values: impl IntoIterator<Item = T>,
    ) -> Self {
        let iter = values.into_iter().map(IntoArrayOfSameType::into_array);

        match self.values {
            Values::Default => self.values = Values::List(iter.collect()),
            Values::List(ref mut vec) => vec.extend(iter),
        }

        self
    }

    pub fn returning(mut self, expressions: impl IntoIteratorOfSameType<OutputExpression>) -> Self {
        self.returning.extend(expressions.into_some_iter());
        self
    }
}

impl<const N: usize> Display for InsertInto<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "INSERT INTO {} ({}) {}",
            self.table_name,
            self.columns.iter().join(", "),
            self.values
        )?;

        if self.returning.len() > 0 {
            write!(f, " RETURNING {}", self.returning.iter().join(", "))?;
        }

        Ok(())
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
        let sql = insert_into("Dummy", ("col1", "col2"))
            .values([("a", "b")])
            .to_string();
        assert_eq!(sql, "INSERT INTO Dummy (col1, col2) VALUES (a, b)");
    }

    #[test]
    fn values_many() {
        let sql = insert_into("Dummy", ("col1", "col2"))
            .values([("a", "b"), ("c", "d")])
            .to_string();
        assert_eq!(sql, "INSERT INTO Dummy (col1, col2) VALUES (a, b), (c, d)");
    }

    #[test]
    fn returning() {
        let sql = insert_into("Dummy", "col1")
            .values(["a"])
            .returning("id")
            .to_string();
        assert_eq!(sql, "INSERT INTO Dummy (col1) VALUES (a) RETURNING id");
    }

    #[test]
    fn returning_two() {
        let sql = insert_into("Dummy", "col1")
            .values(["a"])
            .returning(("id", "place"))
            .to_string();
        assert_eq!(
            sql,
            "INSERT INTO Dummy (col1) VALUES (a) RETURNING id, place"
        );
    }
}