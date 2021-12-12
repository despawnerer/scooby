# Changelog

next
----

- Support arbitrary expressions in `LIMIT` and `OFFSET` clauses, as well as numbers, allowing using parameter placeholders
- Rename `queries` module into `statements`, and use that terminology everywhere
- Add a new convenience function called `from` to create `SELECT` statements by starting from the `FROM` clause
- Fix a broken link in `WithClause` documentation

0.4.0
-----

- Add `Parameters::next_array` function returning placeholder as an array, intended to be used with INSERT INTO queries
- Rename `WithClause::and` to `and_with` for clarity
- Disallow passing `Select` queries directly into `Select::from()` without aliasing them first because PostgreSQL requires it
- Disallow passing raw numbers into `INSERT INTO` and `UPDATE` statements, and update examples to emphasise the raw SQL'ness of everything
- No more dependency on Itertools, making the library totally dependency-free
- No longer allocate strings unnecessarily when we join a bunch of stuff to build SQL

0.3.0
-----

- Write real documentation for most things
- Remove unnecessary `is_empty` method from `WithClause` as the clause should never be empty
- Implement `IntoIteratorOfSameType` and `IntoNonZeroArray` for additional tuples up to 12
- Support `f64`, `i32`, `i64` and `u64` as types convertible into `Expression`
- Remove `Select::new` constructor, use the `select` entry function instead

0.2.0
-----

- Export `BareInsertInto` type from `postgres::queries` module
- Rename `UpdateWithoutAnyValuesSet` into `BareUpdate`
- Support for CTEs (`WITH ...` clauses) in `DELETE FROM`, `UPDATE` and `INSERT_INTO` queries
- Remove `Default` implementations from `DeleteFrom`, `InsertInto` and `Update` structs because such empty structs are not valid
- Implement `Clone` for all query structs so you can, well, clone them
- Mark all query structs as `must_use` rather than initial builder functions
- Relax itertools dependency

0.1.2
-----

- Add basic support for CTEs (`WITH ...`) in `SELECT` queries

0.1.1
-----

- Initial release
