mod column_constraints;
mod column_definition;

use std::fmt::{self, Display, Formatter};

use crate::tools::IntoIteratorOfSameType;
use crate::{postgres::general::TableName, tools::joined};

pub use column_definition::{ColumnDefinition, ColumnDefinitionBuilder, ColumnDefinitionable};

pub fn create_table(table_name: impl Into<TableName>) -> CreateTableBuilder {
    CreateTableBuilder {
        table_name: table_name.into(),
    }
}

pub struct CreateTableBuilder {
    table_name: TableName,
}

impl CreateTableBuilder {
    pub fn columns(self, columns: impl IntoIteratorOfSameType<ColumnDefinition>) -> CreateTable {
        CreateTable {
            name: self.table_name,
            columns: columns.into_some_iter().collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateTable {
    name: TableName,
    columns: Vec<ColumnDefinition>,
}

impl Display for CreateTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CREATE TABLE {} ({})",
            self.name,
            joined(&self.columns, ", ")
        )
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
            .to_string();

        assert_correct_postgresql(&sql, "CREATE TABLE Film (code char(5) PRIMARY KEY, imdb_id char(40) UNIQUE, title varchar(40) NOT NULL, did integer NOT NULL, director_id integer REFERENCES Person(id), date_prod date CHECK (date_prod < today()), kind varchar(10), len interval hour to minute DEFAULT 0)");
    }
}
