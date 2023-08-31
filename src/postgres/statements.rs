//! Specific statement related functions, types and traits.
//!
//! Most likely you want documentation for the [main module][`crate::postgres`].

mod create_table;
mod delete_from;
mod insert_into;
mod select;
mod update;

pub use create_table::{
    create_table, ColumnDefinition, ColumnDefinitionBuilder, ColumnDefinitionable, CreateTable,
    CreateTableBuilder,
};
pub use delete_from::{delete_from, DeleteFrom};
pub use insert_into::{
    insert_into, BareInsertInto, InsertInto, InsertIntoColumnsBuilder, OnConflictClauseBuilder,
    Values,
};
pub use select::{from, select, FromItem, FromSelectBuilder, Joinable, OrderBy, Orderable, Select};
pub use update::{update, BareUpdate, Update};

pub(crate) use delete_from::delete_from_with;
pub(crate) use insert_into::insert_into_with;
pub(crate) use select::select_with;
pub(crate) use update::update_with;
