mod values;

use std::fmt::{self, Display, Formatter};

use itertools::Itertools;

use crate::postgres::general::{Column, Expression, OutputExpression, TableName, WithClause};
use crate::tools::{IntoIteratorOfSameType, IntoNonZeroArray};

pub use values::{DefaultValues, Values, WithColumns, WithoutColumns};

/// Start building a new `INSERT INTO` statement with the given table name.
///
/// Returns a [`BareInsertInto`] structure which requires that you specify
/// what type of a `VALUES` clause you wish to have:
///
/// 1. For `DEFAULT VALUES`, call [`default_values`][BareInsertInto::default_values]
/// 2. For `VALUES (...)` with unspecified columns, call [`values`][BareInsertInto::values]
/// 3. For `(...) VALUES (...)`, call [`columns`][BareInsertInto::columns]
///
/// First two options will give you an [`InsertInto`] structure directly
///
/// Option 3 will expect you to specify at least one set of values through [`values`][InsertIntoColumnsBuilder::values] method
///
/// Call `to_string` on the final `InsertInto` structure to finalize and get an SQL string.
///
/// # Supported clauses
///
/// | Clause      | Method                               |
/// |-------------|--------------------------------------|
/// | `VALUES`    | [`values`][InsertInto::values]       |
/// | `RETURNING` | [`returning`][InsertInto::returning] |
///
/// # Specifying a `WITH` clause
///
/// To create an `INSERT INTO` statement with a `WITH` clause, start with [`with`][crate::postgres::with] instead of this function.
///
/// # Examples
///
/// ```
/// use scooby::postgres::insert_into;
///
/// let sql = insert_into("Dummy").default_values().to_string();
///
/// assert_eq!(sql, "INSERT INTO Dummy DEFAULT VALUES")
/// ```
///
/// ```
/// use scooby::postgres::{insert_into, Parameters};
///
/// let mut params = Parameters::new();
///
/// let sql = insert_into("Rectangle")
///     .columns(("width", "height"))
///     .values([params.next_array()])
///     .returning("id")
///     .to_string();
///
/// assert_eq!(sql, "INSERT INTO Rectangle (width, height) VALUES ($1, $2) RETURNING id");
/// ```
pub fn insert_into(table_name: impl Into<TableName>) -> BareInsertInto {
    BareInsertInto {
        table_name: table_name.into(),
        with: None,
    }
}

pub(crate) fn insert_into_with(table_name: TableName, with: WithClause) -> BareInsertInto {
    BareInsertInto {
        table_name,
        with: Some(with),
    }
}

/// Bare `INSERT INTO` statement without a valid `VALUES` clause specified
///
/// You will want to make use of three methods to convert this into a usable query:
///
/// - [`default_values`][BareInsertInto::default_values] to add a `DEFAULT VALUES` clause
/// - [`values`][BareInsertInto::values] to add `VALUES (...)` clause with unspecified columns
/// - [`columns`][BareInsertInto::columns] to start building a `(...) VALUES (...)` clause with specific columns
#[must_use = "Making a bare INSERT INTO query is pointless"]
#[derive(Debug)]
pub struct BareInsertInto {
    table_name: TableName,
    with: Option<WithClause>,
}

impl BareInsertInto {
    /// Add a `DEFAULT VALUES` clause to this statement
    ///
    /// ```
    /// use scooby::postgres::insert_into;
    ///
    /// let sql = insert_into("Dummy").default_values().to_string();
    ///
    /// assert_eq!(sql, "INSERT INTO Dummy DEFAULT VALUES");
    /// ```
    pub fn default_values(self) -> InsertInto<DefaultValues> {
        InsertInto::new(self.table_name, DefaultValues, self.with)
    }

    /// Add a `VALUES (...)` clause with unspecified columns to this statement
    ///
    /// ```
    /// use scooby::postgres::insert_into;
    ///
    /// let sql = insert_into("Dummy").values([(1, 2), (3, 4)]).to_string();
    ///
    /// assert_eq!(sql, "INSERT INTO Dummy VALUES (1, 2), (3, 4)");
    /// ```
    pub fn values<T: IntoNonZeroArray<Expression, N>, const N: usize>(
        self,
        values: impl IntoIterator<Item = T>,
    ) -> InsertInto<WithoutColumns<N>> {
        let values = values
            .into_iter()
            .map(IntoNonZeroArray::into_non_zero_array)
            .collect();

        InsertInto::new(self.table_name, WithoutColumns::new(values), self.with)
    }

    /// Begin building a `(...) VALUES (...)` clause for this statement.
    ///
    /// Expects a non-zero list of columns: an array, a tuple, or a single value.
    ///
    /// Returns an [`InsertIntoColumnsBuilder`] structure which requires you to specify at least one set of values.
    ///
    /// ```
    /// use scooby::postgres::insert_into;
    ///
    /// let sql = insert_into("Dummy")
    ///     .columns(("col1", "col2"))
    ///     .values([(1, 2), (3, 4)])
    ///     .to_string();
    ///
    /// assert_eq!(sql, "INSERT INTO Dummy (col1, col2) VALUES (1, 2), (3, 4)");
    pub fn columns<const N: usize>(
        self,
        columns: impl IntoNonZeroArray<Column, N>,
    ) -> InsertIntoColumnsBuilder<N> {
        InsertIntoColumnsBuilder {
            table_name: self.table_name,
            with: self.with,
            columns: columns.into_non_zero_array(),
        }
    }
}

/// Intermediate structure to ensure one cannot build an `INSERT INTO` statement with columns, but without values
///
/// Use the only provided [`values`][InsertIntoColumnsBuilder::values] method to add at least one set of values.
#[must_use = "Making a bare INSERT INTO query with columns is pointless"]
#[derive(Debug)]
pub struct InsertIntoColumnsBuilder<const N: usize> {
    table_name: TableName,
    with: Option<WithClause>,
    columns: [Column; N],
}

impl<const N: usize> InsertIntoColumnsBuilder<N> {
    /// Add first one or more sets of values.
    ///
    /// Further values and additional clauses may be added by calling appropriate methods
    /// on the returned [`InsertInto`] structure.
    ///
    /// ```
    /// use scooby::postgres::insert_into;
    ///
    /// let sql = insert_into("Dummy")
    ///     .columns(("col1", "col2"))
    ///     .values([(1, 2), (3, 4)])
    ///     .to_string();
    ///
    /// assert_eq!(sql, "INSERT INTO Dummy (col1, col2) VALUES (1, 2), (3, 4)");
    pub fn values<T: IntoNonZeroArray<Expression, N>>(
        self,
        values: impl IntoIterator<Item = T>,
    ) -> InsertInto<WithColumns<N>> {
        let values = values
            .into_iter()
            .map(IntoNonZeroArray::into_non_zero_array)
            .collect();

        InsertInto::new(
            self.table_name,
            WithColumns::new(self.columns, values),
            self.with,
        )
    }
}

/// `INSERT INTO` statement with a `VALUES` clause, and possibly additional clauses.
///
/// Finalize and turn into `String` by calling `to_string`.
///
/// See [`insert_into`] docs for more details and examples.
#[must_use = "Making an INSERT INTO query without using it is pointless"]
#[derive(Debug, Clone)]
pub struct InsertInto<V: Values> {
    table_name: TableName,
    with: Option<WithClause>,
    values: V,
    returning: Vec<OutputExpression>,
}

impl<V: Values> InsertInto<V> {
    fn new(table_name: TableName, values: V, with: Option<WithClause>) -> InsertInto<V> {
        InsertInto {
            table_name,
            with,
            values,
            returning: Vec::new(),
        }
    }

    /// Add one or more `RETURNING` expressions.
    ///
    /// ```
    /// use scooby::postgres::insert_into;
    ///
    /// let sql = insert_into("Dummy")
    ///     .default_values()
    ///     .returning("id")
    ///     .returning(("width", "height"))
    ///     .to_string();
    ///
    /// assert_eq!(sql, "INSERT INTO Dummy DEFAULT VALUES RETURNING id, width, height");
    /// ```
    pub fn returning(mut self, expressions: impl IntoIteratorOfSameType<OutputExpression>) -> Self {
        self.returning.extend(expressions.into_some_iter());
        self
    }
}

impl<const N: usize> InsertInto<WithColumns<N>> {
    /// Add one or more sets of values.
    ///
    /// ```
    /// use scooby::postgres::insert_into;
    ///
    /// let sql = insert_into("Dummy")
    ///     .columns(("col1", "col2"))
    ///     .values([(1, 2)])
    ///     .values([(3, 4), (5, 6)])
    ///     .to_string();
    ///
    /// assert_eq!(sql, "INSERT INTO Dummy (col1, col2) VALUES (1, 2), (3, 4), (5, 6)");
    pub fn values<T: IntoNonZeroArray<Expression, N>>(
        mut self,
        new_values: impl IntoIterator<Item = T>,
    ) -> Self {
        self.values.add(new_values);
        self
    }
}

impl<const N: usize> InsertInto<WithoutColumns<N>> {
    /// Add one or more sets of values.
    ///
    /// ```
    /// use scooby::postgres::insert_into;
    ///
    /// let sql = insert_into("Dummy")
    ///     .values([(1, 2)])
    ///     .values([(3, 4), (5, 6)])
    ///     .to_string();
    ///
    /// assert_eq!(sql, "INSERT INTO Dummy VALUES (1, 2), (3, 4), (5, 6)");
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
        if let Some(with_clause) = &self.with {
            write!(f, "{} ", with_clause)?;
        }

        write!(f, "INSERT INTO {} {}", self.table_name, self.values)?;

        if self.returning.len() > 0 {
            write!(f, " RETURNING {}", self.returning.iter().join(", "))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::postgres::tools::tests::assert_correct_postgresql;
    use crate::postgres::{insert_into, select, with, Parameters};

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
        let values: [[String; 0]; 1] = [[]];
        let sql = insert_into("Dummy").columns([]).values(values).to_string();

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

    #[test]
    fn cte() {
        let sql = with("thing")
            .as_(select("1 + 1"))
            .insert_into("Dummy")
            .values(["a"])
            .to_string();

        assert_correct_postgresql(
            &sql,
            "WITH thing AS (SELECT 1 + 1) INSERT INTO Dummy VALUES (a)",
        );
    }

    #[test]
    fn array_params_with_columns() {
        let mut params = Parameters::new();

        let sql = insert_into("Dummy")
            .columns(("col1", "col2"))
            .values([params.next_array()])
            .to_string();

        assert_correct_postgresql(&sql, "INSERT INTO Dummy (col1, col2) VALUES ($1, $2)");
    }

    #[test]
    fn array_params_without_columns() {
        let mut params = Parameters::new();

        let sql = insert_into("Dummy")
            .values([params.next_array::<2>()])
            .to_string();

        assert_correct_postgresql(&sql, "INSERT INTO Dummy VALUES ($1, $2)");
    }
}
