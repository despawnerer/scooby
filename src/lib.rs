//! An SQL query builder with a pleasant fluent API closely imitating actual SQL.
//! Meant to comfortably build dynamic queries with a little bit of safety checks sprinkled on
//! top to ensure you don't forget important things like `ON` clauses.
//!
//! See dialect-specific modules for details on supported features and usage.
//!
//! # Requirements
//!
//! Requires Rust 1.54 or later.
//!
//! # Supported dialects
//!
//! - [PostgreSQL][postgres]

pub mod postgres;
mod tools;
