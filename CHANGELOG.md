# Changelog

next
----

- Support for CTEs (`WITH ...` clauses) in `DELETE FROM` and `UPDATE` queries
- `DeleteFrom`, `InsertInto` and `Update` structs now do not implement `Default`, because such empty struts are not valid
- All query structs now implement `Clone` so you can, well, clone them
- All returned query structs are marked as `must_use`
- Relaxed itertools dependency

0.1.2
-----

- Add basic support for CTEs (`WITH ...`) in `SELECT` queries

0.1.1
-----

- Initial release
