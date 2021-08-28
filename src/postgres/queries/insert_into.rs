mod values;

use std::fmt::{self, Display, Formatter};

use itertools::Itertools;

use crate::postgres::general::{Column, Expression, OutputExpression, TableName};
use crate::tools::{IntoIteratorOfSameType, IntoNonZeroArray};

pub use values::{DefaultValues, Values, WithColumns, WithoutColumns};

pub fn insert_into(table_name: impl Into<TableName>) -> BareInsertInto {
    BareInsertInto {
        table_name: table_name.into(),
    }
}

/* Initial INSERT INTO statement without a valid values clause specified */

#[must_use = "Making a bare INSERT INTO query is pointless"]
#[derive(Debug)]
pub struct BareInsertInto {
    table_name: TableName,
}

impl BareInsertInto {
    pub fn default_values(self) -> InsertInto<DefaultValues> {
        InsertInto::new(self.table_name, DefaultValues)
    }

    pub fn values<T: IntoNonZeroArray<Expression, N>, const N: usize>(
        self,
        values: impl IntoIterator<Item = T>,
    ) -> InsertInto<WithoutColumns<N>> {
        let values = values
            .into_iter()
            .map(IntoNonZeroArray::into_non_zero_array)
            .collect();

        InsertInto::new(self.table_name, WithoutColumns::new(values))
    }

    pub fn columns<const N: usize>(
        self,
        columns: impl IntoNonZeroArray<Column, N>,
    ) -> InsertIntoColumnsBuilder<N> {
        InsertIntoColumnsBuilder {
            table_name: self.table_name,
            columns: columns.into_non_zero_array(),
        }
    }
}

/* Intermediate struct to ensure one cannot build an INSERT INTO statement with columns, but without values */

#[must_use = "Making a bare INSERT INTO query with columns is pointless"]
#[derive(Debug)]
pub struct InsertIntoColumnsBuilder<const N: usize> {
    table_name: TableName,
    columns: [Column; N],
}

impl<const N: usize> InsertIntoColumnsBuilder<N> {
    pub fn values<T: IntoNonZeroArray<Expression, N>>(
        self,
        values: impl IntoIterator<Item = T>,
    ) -> InsertInto<WithColumns<N>> {
        let values = values
            .into_iter()
            .map(IntoNonZeroArray::into_non_zero_array)
            .collect();

        InsertInto::new(self.table_name, WithColumns::new(self.columns, values))
    }
}

/* A valid INSERT INTO statement that can already be stringified */

#[must_use = "Making an INSERT INTO query without using it is pointless"]
#[derive(Debug, Clone)]
pub struct InsertInto<V: Values> {
    table_name: TableName,
    values: V,
    returning: Vec<OutputExpression>,
}

impl<V: Values> InsertInto<V> {
    pub fn new(table_name: TableName, values: V) -> InsertInto<V> {
        InsertInto {
            table_name,
            values,
            returning: Vec::new(),
        }
    }

    pub fn returning(mut self, expressions: impl IntoIteratorOfSameType<OutputExpression>) -> Self {
        self.returning.extend(expressions.into_some_iter());
        self
    }
}

impl<const N: usize> InsertInto<WithColumns<N>> {
    pub fn values<T: IntoNonZeroArray<Expression, N>>(
        mut self,
        new_values: impl IntoIterator<Item = T>,
    ) -> Self {
        self.values.add(new_values);
        self
    }
}

impl<const N: usize> InsertInto<WithoutColumns<N>> {
    pub fn values<T: IntoNonZeroArray<Expression, N>>(
        mut self,
        new_values: impl IntoIterator<Item = T>,
    ) -> Self {
        self.values.add(new_values);
        self
    }
}

impl<V: Values> Display for InsertInto<V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "INSERT INTO {} {}", self.table_name, self.values)?;

        if self.returning.len() > 0 {
            write!(f, " RETURNING {}", self.returning.iter().join(", "))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::postgres::insert_into;
    use crate::postgres::tools::tests::assert_correct_postgresql;

    #[test]
    fn default_values() {
        let sql = insert_into("Dummy").default_values().to_string();
        assert_correct_postgresql(&sql, "INSERT INTO Dummy DEFAULT VALUES");
    }

    #[test]
    fn no_columns() {
        let sql = insert_into("Dummy").values(["a"]).to_string();
        assert_correct_postgresql(&sql, "INSERT INTO Dummy VALUES (a)");
    }

    #[test]
    fn no_columns_multiple_values() {
        let sql = insert_into("Dummy")
            .values([("a", "b"), ("c", "d")])
            .values([("e", "f")])
            .to_string();

        assert_correct_postgresql(&sql, "INSERT INTO Dummy VALUES (a, b), (c, d), (e, f)");
    }

    #[test]
    fn no_columns_values_of_different_types() {
        let sql = insert_into("Dummy")
            .values([("\"Doug\"", 5, 1.76)])
            .to_string();

        assert_correct_postgresql(&sql, "INSERT INTO Dummy VALUES (\"Doug\", 5, 1.76)");
    }

    // FIXME: This currently compiles and panics at runtime, but ideally should not even compile
    #[test]
    #[should_panic]
    fn zero_length_columns() {
        let sql = insert_into("Dummy").columns([]).values([[]]).to_string();

        assert_correct_postgresql(&sql, "INSERT INTO Dummy () VALUES ");
    }

    #[test]
    fn single_column() {
        let sql = insert_into("Dummy")
            .columns("col1")
            .values(["a"])
            .to_string();

        assert_correct_postgresql(&sql, "INSERT INTO Dummy (col1) VALUES (a)");
    }

    #[test]
    fn multiple_columns() {
        let sql = insert_into("Dummy")
            .columns(("col1", "col2"))
            .values([("a", "b")])
            .to_string();

        assert_correct_postgresql(&sql, "INSERT INTO Dummy (col1, col2) VALUES (a, b)");
    }

    #[test]
    fn many_values() {
        let sql = insert_into("Dummy")
            .columns(("col1", "col2"))
            .values([("a", "b"), ("c", "d")])
            .values([("e", "f")])
            .to_string();

        assert_correct_postgresql(
            &sql,
            "INSERT INTO Dummy (col1, col2) VALUES (a, b), (c, d), (e, f)",
        );
    }

    #[test]
    fn value_various_types() {
        let sql = insert_into("Dummy")
            .columns(("name", "age", "height_in_meters"))
            .values([("\"Doug\"", 5, 1.76)])
            .to_string();

        assert_correct_postgresql(
            &sql,
            "INSERT INTO Dummy (name, age, height_in_meters) VALUES (\"Doug\", 5, 1.76)",
        );
    }

    #[test]
    fn returning() {
        let sql = insert_into("Dummy")
            .columns("col1")
            .values(["a"])
            .returning("id")
            .to_string();

        assert_correct_postgresql(&sql, "INSERT INTO Dummy (col1) VALUES (a) RETURNING id");
    }

    #[test]
    fn returning_two() {
        let sql = insert_into("Dummy")
            .columns("col1")
            .values(["a"])
            .returning(("id", "place"))
            .to_string();

        assert_correct_postgresql(
            &sql,
            "INSERT INTO Dummy (col1) VALUES (a) RETURNING id, place",
        );
    }
}
