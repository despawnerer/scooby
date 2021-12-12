//! Implementation of PostgreSQL dialect of SQL, including some PostgreSQL specific features and tools.
//!
//! # Supported statements
//!
//! See each function's docs for details on supported clauses and features.
//!
//! | Entry function  | SQL statement                          |
//! |-----------------|----------------------------------------|
//! | [`select`]      | `SELECT`                               |
//! | [`from`]        | `SELECT` (starting from `FROM` clause) |
//! | [`insert_into`] | `INSERT INTO`                          |
//! | [`delete_from`] | `DELETE FROM`                          |
//! | [`update`]      | `UPDATE`                               |
//! | [`with`]        | `WITH`                                 |
//!
//! # Tools
//!
//! | Tool           | Description                               |
//! |----------------|-------------------------------------------|
//! | [`Parameters`] | Generator of query parameter placeholders |
//!
pub mod general;
pub mod statements;
pub mod tools;

pub use general::{with, Aliasable};
pub use statements::{
    delete_from, from, insert_into, select, update, DeleteFrom, FromSelectBuilder, InsertInto,
    Joinable, Orderable, Select, Update,
};
pub use tools::Parameters;
