mod distinct;
mod from_item;
mod order_by;
mod join;

use std::default::Default;
use std::fmt::{self, Display, Formatter};

use itertools::Itertools;

use crate::general::{Condition, Expression};
use crate::tools::IntoIteratorOfSameType;

pub use distinct::Distinct;
pub use from_item::FromItem;
pub use join::Joinable;
pub use order_by::{OrderBy, Orderable};

#[must_use = "Making a query without using it pointless"]
pub fn select(expressions: impl IntoIteratorOfSameType<Expression>) -> Select {
    Select {
        expressions: expressions.into_some_iter().collect(),
        ..Default::default()
    }
}

#[derive(Default, Debug)]
pub struct Select {
    expressions: Vec<Expression>,
    from: Vec<FromItem>,
    where_: Vec<Condition>,
    group_by: Vec<Expression>,
    having: Vec<Condition>,
    order_by: Vec<OrderBy>,
    limit: Option<u64>,
    offset: Option<u64>,
    distinct: Option<Distinct>,
}

impl Select {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn and_select(mut self, expressions: impl IntoIteratorOfSameType<Expression>) -> Self {
        self.expressions.extend(expressions.into_some_iter());
        self
    }

    pub fn all(mut self) -> Self {
        self.distinct = Some(Distinct::All);
        self
    }

    pub fn distinct(mut self) -> Self {
        self.distinct = Some(Distinct::Distinct);
        self
    }

    pub fn distinct_on(mut self, expressions: impl IntoIteratorOfSameType<Expression>) -> Self {
        self.distinct = Some(Distinct::DistinctOn(expressions.into_some_iter().collect()));
        self
    }

    pub fn from(mut self, from: impl IntoIteratorOfSameType<FromItem>) -> Self {
        self.from.extend(from.into_some_iter());
        self
    }

    pub fn where_(mut self, conditions: impl IntoIteratorOfSameType<Condition>) -> Self {
        self.where_.extend(conditions.into_some_iter());
        self
    }

    pub fn group_by(mut self, groupings: impl IntoIteratorOfSameType<Expression>) -> Self {
        self.group_by.extend(groupings.into_some_iter());
        self
    }

    pub fn having(mut self, conditions: impl IntoIteratorOfSameType<Condition>) -> Self {
        self.having.extend(conditions.into_some_iter());
        self
    }

    pub fn order_by(mut self, order_bys: impl IntoIteratorOfSameType<OrderBy>) -> Self {
        self.order_by.extend(order_bys.into_some_iter());
        self
    }

    pub fn limit(mut self, n: u64) -> Self {
        self.limit = Some(n);
        self
    }

    pub fn offset(mut self, n: u64) -> Self {
        self.offset = Some(n);
        self
    }
}

impl Display for Select {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "SELECT")?;

        if let Some(distinct) = &self.distinct {
            write!(f, " {}", distinct)?;
        }

        if self.expressions.len() > 0 {
            write!(f, " {}", self.expressions.iter().join(", "))?; // TODO: can be done without creating a temporary string?
        }

        if self.from.len() > 0 {
            write!(f, " FROM {}", self.from.iter().join(", "))?;
        }

        if self.where_.len() > 0 {
            write!(f, " WHERE {}", self.where_.iter().join(" AND "))?;
        }

        if self.group_by.len() > 0 {
            write!(f, " GROUP BY {}", self.group_by.iter().join(", "))?;
        }

        if self.having.len() > 0 {
            write!(f, " HAVING {}", self.having.iter().join(" AND "))?;
        }

        if self.order_by.len() > 0 {
            write!(f, " ORDER BY {}", self.order_by.iter().join(", "))?;
        }

        if let Some(limit) = self.limit {
            write!(f, " LIMIT {}", limit)?;
        }

        if let Some(offset) = self.offset {
            write!(f, " OFFSET {}", offset)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::tools::tests::assert_correct_postgresql;
    use crate::{select, Aliasable, Joinable, Orderable};

    #[test]
    fn bare() {
        let sql = select(()).to_string();
        assert_correct_postgresql(&sql, "SELECT");
    }

    #[test]
    fn without_from() {
        let sql = select("1 + 1").to_string();
        assert_correct_postgresql(&sql, "SELECT 1 + 1");
    }

    #[test]
    fn all() {
        let sql = select("*").all().from("City").to_string();
        assert_correct_postgresql(&sql, "SELECT ALL * FROM City");
    }

    #[test]
    fn distinct() {
        let sql = select("*").distinct().from("City").to_string();
        assert_correct_postgresql(&sql, "SELECT DISTINCT * FROM City");
    }

    #[test]
    fn tuple_of_columns() {
        let sql = select(("id", "name")).from("Person").to_string();
        assert_correct_postgresql(&sql, "SELECT id, name FROM Person")
    }

    #[test]
    fn slice_of_columns() {
        let sql = select(&["id", "name"]).from("Person").to_string();
        assert_correct_postgresql(&sql, "SELECT id, name FROM Person")
    }

    #[test]
    fn array_of_columns() {
        let sql = select(["id", "name"]).from("Person").to_string();
        assert_correct_postgresql(&sql, "SELECT id, name FROM Person")
    }

    #[test]
    fn no_columns() {
        let sql = select(()).from("Person").to_string();
        assert_correct_postgresql(&sql, "SELECT FROM Person");
    }

    #[test]
    fn and_select() {
        let sql = select(("id", "name"))
            .from("Person")
            .and_select("age")
            .and_select(("occupation_id", "city_id"))
            .to_string();

        assert_correct_postgresql(
            &sql,
            "SELECT id, name, age, occupation_id, city_id FROM Person",
        )
    }

    #[test]
    fn from_single_table() {
        let sql = select("name").from("Person").to_string();
        assert_correct_postgresql(&sql, "SELECT name FROM Person");
    }

    #[test]
    fn from_twice() {
        let sql = select("*").from("OneTable").from("OtherTable").to_string();
        assert_correct_postgresql(&sql, "SELECT * FROM OneTable, OtherTable");
    }

    #[test]
    fn from_alias() {
        let sql = select("*").from("Person".as_("p")).to_string();
        assert_correct_postgresql(&sql, "SELECT * FROM Person AS p");
    }

    #[test]
    fn from_tuple_of_tables() {
        let sql = select(&["p.name", "c.name", "d.name"])
            .from(("Person p", "City c", "District d"))
            .to_string();

        assert_correct_postgresql(
            &sql,
            "SELECT p.name, c.name, d.name FROM Person p, City c, District d",
        );
    }

    #[test]
    fn from_join() {
        let sql = select("col1")
            .from("Person p".join("City c").on("c.id = p.city_id"))
            .to_string();

        assert_correct_postgresql(
            &sql,
            "SELECT col1 FROM Person p JOIN City c ON c.id = p.city_id",
        );
    }

    #[test]
    fn from_join_with_alias() {
        let sql = select("*")
            .from(
                "Person"
                    .as_("p")
                    .join("City".as_("c"))
                    .on("c.id = p.city_id"),
            )
            .to_string();

        assert_correct_postgresql(
            &sql,
            "SELECT * FROM Person AS p JOIN City AS c ON c.id = p.city_id",
        );
    }

    #[test]
    fn from_multiple_joins() {
        let sql = select("col1")
            .from(
                "Person p"
                    .inner_join("City c")
                    .on("c.id = p.city_id")
                    .left_join("Belonging b")
                    .on("p.id = b.person_id"),
            )
            .to_string();

        assert_correct_postgresql(&sql, "SELECT col1 FROM Person p INNER JOIN City c ON c.id = p.city_id LEFT JOIN Belonging b ON p.id = b.person_id");
    }

    #[test]
    fn cross_join() {
        let sql = select("*").from("One".cross_join("Two")).to_string();

        assert_correct_postgresql(&sql, "SELECT * FROM One CROSS JOIN Two");
    }

    #[test]
    fn cross_join_chain() {
        let sql = select("*")
            .from("One".cross_join("Two").cross_join("Three"))
            .to_string();

        assert_correct_postgresql(&sql, "SELECT * FROM One CROSS JOIN Two CROSS JOIN Three");
    }

    #[test]
    fn nested_join_madness() {
        let sql = select("*")
            .from(
                "t1".left_join("t2".cross_join("t3").cross_join("t4"))
                    .on("(t2.a = t1.a AND t3.b = t1.b AND t4.c = t1.c)"),
            )
            .to_string();

        assert_correct_postgresql(&sql, "SELECT * FROM t1 LEFT JOIN (t2 CROSS JOIN t3 CROSS JOIN t4) ON (t2.a = t1.a AND t3.b = t1.b AND t4.c = t1.c)");
    }

    #[test]
    fn from_heterogeneous_tables() {
        let sql = select("*")
            .from((
                "Person p".inner_join("City c").on("c.id = p.city_id"),
                "OtherTable o",
            ))
            .to_string();

        assert_correct_postgresql(
            &sql,
            "SELECT * FROM Person p INNER JOIN City c ON c.id = p.city_id, OtherTable o",
        )
    }

    #[test]
    fn from_subselect() {
        let sql = select("*")
            .from(select("id").from("City").as_("x"))
            .to_string();

        // FIXME: would be nice to guarantee that subselects MUST be aliased somehow?
        assert_correct_postgresql(&sql, "SELECT * FROM (SELECT id FROM City) AS x");
    }

    #[test]
    fn from_subselect_with_alias() {
        let subselect = select(("id", "planet_id")).from("City");
        let sql = select("*")
            .from(
                subselect
                    .as_("c")
                    .inner_join("Planet".as_("p"))
                    .on("c.planet_id = p.id"),
            )
            .to_string();

        assert_correct_postgresql(&sql, "SELECT * FROM (SELECT id, planet_id FROM City) AS c INNER JOIN Planet AS p ON c.planet_id = p.id");
    }

    #[test]
    fn group_by() {
        let sql = select(("country_id", "COUNT(*)"))
            .from("City")
            .group_by("country_id")
            .to_string();

        assert_correct_postgresql(
            &sql,
            "SELECT country_id, COUNT(*) FROM City GROUP BY country_id",
        );
    }

    #[test]
    fn order_by() {
        let sql = select("*").from("City").order_by("id").to_string();
        assert_correct_postgresql(&sql, "SELECT * FROM City ORDER BY id");
    }

    #[test]
    fn order_by_two() {
        let sql = select("*")
            .from("City")
            .order_by(("country_id", "id"))
            .to_string();

        assert_correct_postgresql(&sql, "SELECT * FROM City ORDER BY country_id, id");
    }

    #[test]
    fn order_by_desc() {
        let sql = select("*").from("City").order_by("id".desc()).to_string();
        assert_correct_postgresql(&sql, "SELECT * FROM City ORDER BY id DESC");
    }

    #[test]
    fn limit() {
        let sql = select("whatever").from("SomeTable").limit(5).to_string();
        assert_correct_postgresql(&sql, "SELECT whatever FROM SomeTable LIMIT 5");
    }

    #[test]
    fn offset() {
        let sql = select("whatever").from("SomeTable").offset(5).to_string();
        assert_correct_postgresql(&sql, "SELECT whatever FROM SomeTable OFFSET 5");
    }

    #[test]
    fn limit_with_offset() {
        let sql = select("whatever")
            .from("SomeTable")
            .limit(10)
            .offset(5)
            .to_string();

        assert_correct_postgresql(&sql, "SELECT whatever FROM SomeTable LIMIT 10 OFFSET 5");
    }
}
