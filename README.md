Scooby
======

An SQL query builder with a pleasant fluent API closely imitating actual SQL. Meant to comfortably build dynamic queries with a little bit of safety checks sprinkled on top to ensure you don't forget important things like `ON` clauses. Does not do quoting, does not do validation.

Supports only PostgreSQL syntax.

Requires Rust 1.54.


Supported statements, clauses and features
------------------------------------------

1. `SELECT`
    - `WHERE`
    - `GROUP BY`
    - `HAVING`
    - `ALL`, `DISTINCT` and `DISTINCT ON`
    - `ORDER BY`
    - `LIMIT` and `OFFSET`
    - `FROM` with subselects and joins with a nice API:
        - `JOIN`, `INNER JOIN` and `CROSS JOIN`
        - `LEFT JOIN` and `LEFT OUTER JOIN`
        - `RIGHT JOIN` and `RIGHT OUTER JOIN`
        - `FULL JOIN` and `FULL OUTER JOIN`

2. `INSERT INTO`
    - `DEFAULT VALUES`
    - `VALUES` with compile-time checking that lengths of all values are the same as columns
    - `RETURNING`

3. `DELETE FROM`
    - `WHERE`
    - `RETURNING`

4. `UPDATE`
    - `SET` with compile-time checking that you've actually set at least something
    - `WHERE`
    - `RETURNING`

5. Convenient `x AS y` aliasing

6. Convenient `$1`, `$2`... parameter placeholder builder

Examples
--------

### `SELECT`

```rust
use scooby::postgres::{select, Aliasable, Joinable, Orderable};

// SELECT 1 + 1
select("1 + 1").to_string();

// SELECT id, name, age FROM Person
select(("id", "name", "age")).from("Person").to_string();

// SELECT * FROM (SELECT id FROM City) AS x
select("*")
    .from(select("id").from("City").as_("x"))
    .to_string();

// SELECT col1, col2 FROM SomeTable LIMIT 10 OFFSET 5
select("col1")
    .and_select("col2")
    .from("SomeTable")
    .limit(10)
    .offset(5)
    .to_string();

// SELECT
//     country.name AS name,
//     COUNT(*) AS count
// FROM
//     Country AS country
//     INNER JOIN City AS city ON city.country_id = country.id
// WHERE
//     city.population > 1000000
// GROUP BY country.id
// ORDER BY count DESC
// LIMIT 10
select(("country.name".as_("name"), "COUNT(*)".as_("count")))
    .from(
        "Country"
            .as_("country")
            .inner_join("City".as_("city"))
            .on("city.country_id = country.id"),
    )
    .where_("city.population > 1000000")
    .group_by("country.id")
    .order_by("count".desc())
    .limit(10)
    .to_string();
```

### `INSERT INTO`

```rust
use scooby::postgres::insert_into;

// INSERT INTO Dummy (col1, col2) VALUES (a, b), (c, d), (e, f) RETURNING id
insert_into("Dummy")
    .columns(("col1", "col2"))
    .values([("a", "b"), ("c", "d")])
    .values([("e", "f")])
    .returning("id")
    .to_string();

// INSERT INTO Dummy DEFAULT VALUES
insert_into("Dummy").default_values().to_string();
```

### `DELETE FROM`

```rust
use scooby::postgres::delete_from;

// DELETE FROM Dummy WHERE x > 0 AND y > 30
delete_from("Dummy").where_(("x > 0", "y > 30")).to_string();
```

### `Parameters`

```rust
use scooby::postgres::{select, Parameters};

let mut params = Parameters::new();

// SELECT id FROM Thing WHERE x > $1 AND y < $2
select("id")
    .from("Thing")
    .where_(format!("x > {}", params.next()))
    .where_(format!("y < {}", params.next()))
    .to_string();
```

Testing
-------

Normally:

```bash
cargo test
```

To check syntax:

1. Run a local postgresql server on your machine at default port
2. `cargo test --features validate-postgres-syntax`
