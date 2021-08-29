//! An SQL query builder with a pleasant fluent API closely imitating actual SQL.
//! Meant to comfortably build dynamic queries with a little bit of safety checks sprinkled on
//! top to ensure you don't forget important things like `ON` clauses.
//!
//! See dialect-specific modules for details on supported features and usage.
//!
//! # Supported dialects
//!
//! - [PostgreSQL][postgres]
//!
//! # Quick example
//!
//! ```
//! use scooby::postgres::{select, Aliasable, Orderable, Joinable};
//!
//! // SELECT
//! //     country.name AS name,
//! //     COUNT(*) AS count
//! // FROM
//! //     Country AS country
//! //     INNER JOIN City AS city ON city.country_id = country.id
//! // WHERE
//! //     city.population > 1000000
//! // GROUP BY country.name
//! // ORDER BY count DESC
//! // LIMIT 10
//! select(("country.name".as_("name"), "COUNT(*)".as_("count")))
//!     .from(
//!         "Country"
//!             .as_("country")
//!             .inner_join("City".as_("city"))
//!             .on("city.country_id = country.id"),
//!     )
//!     .where_("city.population > 1000000")
//!     .group_by("country.name")
//!     .order_by("count".desc())
//!     .limit(10)
//!     .to_string();
//! ```
//!
//! # Requirements
//!
//! Requires Rust 1.54 or later.
pub mod postgres;
mod tools;
