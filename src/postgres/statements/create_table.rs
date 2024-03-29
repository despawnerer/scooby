mod column_constraints;
mod column_definition;
mod table_constraints;

use std::fmt::{self, Display, Formatter};

use crate::postgres::general::Column;
use crate::tools::IntoIteratorOfSameType;
use crate::{postgres::general::TableName, tools::joined};

pub use column_definition::{ColumnDefinition, ColumnDefinitionBuilder, ColumnDefinitionable};

use self::table_constraints::TableConstraint;

pub fn create_table(table_name: impl Into<TableName>) -> CreateTableBuilder {
    CreateTableBuilder {
        table_name: table_name.into(),
        if_not_exists: false,
    }
}

pub struct CreateTableBuilder {
    table_name: TableName,
    if_not_exists: bool,
}

impl CreateTableBuilder {
    pub fn if_not_exists(mut self) -> CreateTableBuilder {
        self.if_not_exists = true;
        self
    }

    pub fn columns(self, columns: impl IntoIteratorOfSameType<ColumnDefinition>) -> CreateTable {
        CreateTable {
            name: self.table_name,
            if_not_exists: self.if_not_exists,
            columns: columns.into_some_iter().collect(),
            constraints: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateTable {
    name: TableName,
    if_not_exists: bool,
    columns: Vec<ColumnDefinition>,
    constraints: Vec<TableConstraint>,
}

impl Display for CreateTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "CREATE TABLE")?;

        if self.if_not_exists {
            write!(f, " IF NOT EXISTS")?;
        }

        write!(f, " {} ({}", self.name, joined(&self.columns, ", "))?;

        if self.constraints.len() > 0 {
            write!(f, ", {}", joined(&self.constraints, ", "))?;
        }

        write!(f, ")")
    }
}

impl CreateTable {
    pub fn unique(mut self, columns: impl IntoIteratorOfSameType<Column>) -> Self {
        self.constraints
            .push(TableConstraint::Unique(columns.into_some_iter().collect()));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::postgres::tools::tests::assert_correct_postgresql;
    use crate::postgres::{create_table, ColumnDefinitionable};

    #[test]
    fn regular_table() {
        let sql = create_table("Film")
            .columns((
                ("code", "char(5)").primary_key(),
                ("imdb_id", "char(40)").unique(),
                ("title", "varchar(40)").not_null(),
                ("did", "integer").not_null(),
                ("director_id", "integer").references("Person", "id"),
                ("date_prod", "date").check("date_prod < today()"),
                ("kind", "varchar(10)"),
                ("len", "interval hour to minute").default("0"),
            ))
            .unique("code")
            .to_string();

        assert_correct_postgresql(&sql, "CREATE TABLE Film (code char(5) PRIMARY KEY, imdb_id char(40) UNIQUE, title varchar(40) NOT NULL, did integer NOT NULL, director_id integer REFERENCES Person(id), date_prod date CHECK (date_prod < today()), kind varchar(10), len interval hour to minute DEFAULT 0, UNIQUE (code))");
    }

    #[test]
    fn if_not_exists() {
        let sql = create_table("Dummy")
            .if_not_exists()
            .columns((("a", "integer"),))
            .to_string();

        assert_correct_postgresql(&sql, "CREATE TABLE IF NOT EXISTS Dummy (a integer)");
    }
}
