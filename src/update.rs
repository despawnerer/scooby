use std::fmt::{self, Display, Formatter};

use itertools::Itertools;

use crate::general::{Column, Condition, Expression, OutputExpression};
use crate::tools::IntoIteratorOfSameType;

pub fn update(table_name: &str) -> UpdateWithoutAnyValuesSet {
    UpdateWithoutAnyValuesSet {
        table_name: table_name.to_owned(),
    }
}

#[derive(Debug)]
pub struct UpdateWithoutAnyValuesSet {
    table_name: String,
}

impl UpdateWithoutAnyValuesSet {
    pub fn set<C: Into<Column>, V: Into<Expression>>(self, column: C, value: V) -> Update {
        let mut values = Vec::new();
        values.push((column.into(), value.into()));

        Update {
            table_name: self.table_name,
            values,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct Update {
    table_name: String,
    values: Vec<(Column, Expression)>,
    where_: Vec<Condition>,
    returning: Vec<OutputExpression>,
}

impl Update {
    pub fn set<C: Into<Column>, V: Into<Expression>>(mut self, column: C, value: V) -> Self {
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
    use crate::update;

    #[test]
    fn update_single_value() {
        let sql = update("Dummy").set("x", "y").to_string();
        assert_eq!(sql, "UPDATE Dummy SET x = y");
    }

    #[test]
    fn update_multi_call() {
        let sql = update("Dummy").set("x", "y").set("a", "b").to_string();
        assert_eq!(sql, "UPDATE Dummy SET x = y, a = b");
    }

    #[test]
    fn update_where() {
        let sql = update("Dummy").set("x", "y").where_("id = 5").to_string();
        assert_eq!(sql, "UPDATE Dummy SET x = y WHERE id = 5");
    }

    #[test]
    fn update_returning() {
        let sql = update("Dummy").set("x", "y").returning("x").to_string();
        assert_eq!(sql, "UPDATE Dummy SET x = y RETURNING x");
    }
}
