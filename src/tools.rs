mod into_array_of_same_type;
mod into_iterator_of_same_type;
mod parameters;

pub use into_array_of_same_type::IntoArrayOfSameType;
pub use into_iterator_of_same_type::IntoIteratorOfSameType;
pub use parameters::Parameters;

#[cfg(test)]
pub mod tests {
    #[cfg(feature = "validate-postgres-syntax")]
    use postgres::{error::SqlState, Client, NoTls};

    pub fn assert_correct_postgresql(sql: &str, expected: &str) {
        assert_eq!(sql, expected);

        #[cfg(feature = "validate-postgres-syntax")]
        {
            let mut client = Client::connect("host=localhost user=postgres", NoTls).unwrap();
            match client.prepare(sql) {
                Ok(_) => return,
                Err(e) => {
                    if e.code() == Some(&SqlState::SYNTAX_ERROR) {
                        panic!("invalid SQL syntax: {}", e.as_db_error().unwrap().message())
                    }
                }
            };
        }
    }
}
