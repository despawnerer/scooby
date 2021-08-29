# Changelog

next
----
- Implement `IntoIteratorOfSameType` and `IntoNonZeroArray` for additional tuples up to 12

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
