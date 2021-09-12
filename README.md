Scooby
======

[![Latest Version](https://img.shields.io/crates/v/scooby.svg)](https://crates.io/crates/scooby)
[![docs](https://docs.rs/scooby/badge.svg)](https://docs.rs/scooby)

An SQL query builder with a pleasant fluent API closely imitating actual SQL. Meant to comfortably build dynamic queries with a little bit of safety checks sprinkled on top to ensure you don't forget important things like `ON` clauses. Does not do quoting, does not do validation.

Supports only PostgreSQL syntax at the moment.

Requires Rust 1.54.

Principles
----------

- Single responsibility: _builds SQL queries_. Everything else is out of scope.
- API designed to look as _close to actual SQL_ as possible, while being a tiny bit more flexible.
- Everything is _raw SQL strings_. If you need to pass user input, please use parametrized queries.
- Obvious _mistakes should be prevented_ at compile time, where possible.
- No external dependencies

Supported statements, clauses and features
------------------------------------------

1. `SELECT`
    - `WITH`
    - `WHERE`
    - `GROUP BY`
    - `HAVING`
    - `ALL`, `DISTINCT` and `DISTINCT ON`
    - `ORDER BY`
        - `ASC`
        - `DESC`
        - `NULLS FIRST`
        - `NULLS LAST`
    - `LIMIT` and `OFFSET`
    - `FROM` with subselects and joins with a nice API:
        - `JOIN`, `INNER JOIN` and `CROSS JOIN`
        - `LEFT JOIN` and `LEFT OUTER JOIN`
        - `RIGHT JOIN` and `RIGHT OUTER JOIN`
        - `FULL JOIN` and `FULL OUTER JOIN`

2. `INSERT INTO`
    - `WITH`
    - `DEFAULT VALUES`
    - `VALUES` with compile-time checking that lengths of all values are the same as columns
    - `RETURNING`

3. `DELETE FROM`
    - `WITH`
    - `WHERE`
    - `RETURNING`

4. `UPDATE`
    - `WITH`
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

### `WITH` (CTE — Common Table Expression)

```rust
use scooby::postgres::{with, select};

// WITH regional_sales AS (
//         SELECT region, SUM(amount) AS total_sales
//         FROM orders
//         GROUP BY region
//      ), top_regions AS (
//         SELECT region
//         FROM regional_sales
//         WHERE total_sales > (SELECT SUM(total_sales)/10 FROM regional_sales)
//      )
// SELECT region,
//        product,
//        SUM(quantity) AS product_units,
//        SUM(amount) AS product_sales
// FROM orders
// WHERE region IN (SELECT region FROM top_regions)
// GROUP BY region, product;
with("regional_sales")
    .as_(
        select(("region", "SUM(amount)".as_("total_sales")))
            .from("orders")
            .group_by("region"),
    )
    .and("top_regions")
    .as_(select("region").from("regional_sales").where_(format!(
        "total_sales > ({})",
        select("SUM(total_sales)/10").from("regional_sales")
    )))
    .select((
        "region",
        "product",
        "SUM(quantity)".as_("product_units"),
        "SUM(amount)".as_("product_sales"),
    ))
    .from("orders")
    .where_(format!(
        "region IN ({})",
        select("region").from("top_regions")
    ))
    .group_by(("region", "product"))
    .to_string();
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
