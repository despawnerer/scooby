//! Implementation of PostgreSQL dialect of SQL, including some PostgreSQL specific features and tools.
//!
//! # Supported statements
//!
//! See each function's docs for details on supported clauses and features.
//!
//! | Entry function   | SQL statement                          |
//! |------------------|----------------------------------------|
//! | [`select`]       | `SELECT`                               |
//! | [`from`]         | `SELECT` (starting from `FROM` clause) |
//! | [`insert_into`]  | `INSERT INTO`                          |
//! | [`delete_from`]  | `DELETE FROM`                          |
//! | [`update`]       | `UPDATE`                               |
//! | [`with`]         | `WITH`                                 |
//! | [`create_table`] | `CREATE TABLE`                         |
//!
//! # Tools
//!
//! | Tool           | Description                                   |
//! |----------------|-----------------------------------------------|
//! | [`Parameters`] | Generator of statement parameter placeholders |
//!
pub mod general;
pub mod statements;
pub mod tools;

pub use general::{with, Aliasable};
pub use statements::{
    create_table, delete_from, from, insert_into, select, update, ColumnDefinitionable,
    CreateTable, DeleteFrom, FromSelectBuilder, InsertInto, Joinable, Orderable, Select, Update,
};
pub use tools::Parameters;
