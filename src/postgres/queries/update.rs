use std::fmt::{self, Display, Formatter};

use itertools::Itertools;

use crate::postgres::general::{Column, Condition, Expression, OutputExpression, TableName, WithClause};
use crate::tools::IntoIteratorOfSameType;

pub fn update(table_name: impl Into<TableName>) -> UpdateWithoutAnyValuesSet {
    UpdateWithoutAnyValuesSet {
        table_name: table_name.into(),
        with: None
    }
}

pub(crate) fn update_with(table_name: TableName, with: WithClause) -> UpdateWithoutAnyValuesSet {
    UpdateWithoutAnyValuesSet {
        table_name,
        with: Some(with)
    }
}

#[must_use = "Making an UPDATE query with no values set is pointless"]
#[derive(Debug)]
pub struct UpdateWithoutAnyValuesSet {
    table_name: TableName,
    with: Option<WithClause>
}

impl UpdateWithoutAnyValuesSet {
    pub fn set(self, column: impl Into<Column>, value: impl Into<Expression>) -> Update {
        Update::new(self.table_name, vec![(column.into(), value.into())], self.with)
    }
}

#[must_use = "Making an UPDATE query without using it is pointless"]
#[derive(Debug, Clone)]
pub struct Update {
    table_name: TableName,
    with: Option<WithClause>,
    values: Vec<(Column, Expression)>,
    where_: Vec<Condition>,
    returning: Vec<OutputExpression>,
}

impl Update {
    pub fn new(table_name: TableName, values: Vec<(Column, Expression)>, with: Option<WithClause>) -> Update {
        Update {
            table_name,
            values,
            with,
            where_: Vec::new(),
            returning: Vec::new(),
        }
    }

    pub fn set(mut self, column: impl Into<Column>, value: impl Into<Expression>) -> Self {
        self.values.push((column.into(), value.into()));
        self
    }

    pub fn where_(mut self, conditions: impl IntoIteratorOfSameType<Condition>) -> Self {
        self.where_.extend(conditions.into_some_iter());
        self
    }

    pub fn returning(mut self, expressions: impl IntoIteratorOfSameType<OutputExpression>) -> Self {
        self.returning.extend(expressions.into_some_iter());
        self
    }
}

impl Display for Update {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(with_clause) = &self.with {
            write!(f, "{} ", with_clause)?;
        }

        write!(
            f,
            "UPDATE {} SET {}",
            self.table_name,
            self.values
                .iter()
                .map(|(col, val)| format!("{} = {}", col, val))
                .join(", ")
        )?;

        if self.where_.len() > 0 {
            write!(f, " WHERE {}", self.where_.iter().join(" AND "))?;
        }

        if self.returning.len() > 0 {
            write!(f, " RETURNING {}", self.returning.iter().join(", "))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::postgres::tools::tests::assert_correct_postgresql;
    use crate::postgres::{select, with, update};

    #[test]
    fn update_single_value() {
        let sql = update("Dummy").set("x", "y").to_string();
        assert_correct_postgresql(&sql, "UPDATE Dummy SET x = y");
    }

    #[test]
    fn update_multi_call() {
        let sql = update("Dummy").set("x", "y").set("a", "b").to_string();
        assert_correct_postgresql(&sql, "UPDATE Dummy SET x = y, a = b");
    }

    #[test]
    fn update_where() {
        let sql = update("Dummy").set("x", "y").where_("id = 5").to_string();
        assert_correct_postgresql(&sql, "UPDATE Dummy SET x = y WHERE id = 5");
    }

    #[test]
    fn update_returning() {
        let sql = update("Dummy").set("x", "y").returning("x").to_string();
        assert_correct_postgresql(&sql, "UPDATE Dummy SET x = y RETURNING x");
    }

    #[test]
    fn cte() {
        let sql = with("thing")
            .as_(select("1 + 1"))
            .update("Dummy")
            .set("x", "y")
            .to_string();

        assert_correct_postgresql(&sql, "WITH thing AS (SELECT 1 + 1) UPDATE Dummy SET x = y");
    }
}
