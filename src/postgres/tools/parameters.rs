use crate::tools::{build_array, joined};

/// Generator of PostgreSQL parameter placeholders for dynamic statements with multiple values
///
/// # Example
///
/// ```
/// use scooby::postgres::Parameters;
///
/// let mut params = Parameters::new();
/// let p1 = params.next();
/// let p2 = params.next();
/// let p345 = params.next_n(3);
/// let p67 = params.next_array::<2>();
///
/// assert_eq!(p1, "$1");
/// assert_eq!(p2, "$2");
/// assert_eq!(p345, "$3, $4, $5");
/// assert_eq!(p67, ["$6", "$7"]);
/// ```
pub struct Parameters {
    current: usize,
}

impl Parameters {
    /// Make a new Parameters counter, starting with 1
    pub fn new() -> Parameters {
        Parameters { current: 1 }
    }

    /// Make a new Parameters counter, starting with passed number
    pub fn starting_from(first: usize) -> Parameters {
        Parameters { current: first }
    }

    /// Return the current parameter placeholder in `$x` format, and increase the internal counter
    pub fn next(&mut self) -> String {
        let s = format!("${}", self.current);
        self.current += 1;
        s
    }

    /// Return N next placeholders in `$x, $y, $z` format
    pub fn next_n(&mut self, n: usize) -> String {
        let last = self.current + n;
        // TODO: This allocates a bunch of strings totally unnecessarily
        let s = joined((self.current..last).map(|x| format!("${}", x)), ", ").to_string();
        self.current = last;
        s
    }

    /// Return N next placeholders as an array of size N
    pub fn next_array<const N: usize>(&mut self) -> [String; N] {
        build_array(|| self.next())
    }
}

impl Default for Parameters {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_once() {
        let mut params = Parameters::new();
        assert_eq!(params.next(), "$1");
    }

    #[test]
    fn next_twice() {
        let mut params = Parameters::new();
        params.next();
        assert_eq!(params.next(), "$2");
    }

    #[test]
    fn next_n_once() {
        let mut params = Parameters::new();
        assert_eq!(params.next_n(5), "$1, $2, $3, $4, $5");
    }

    #[test]
    fn next_n_with_one() {
        let mut params = Parameters::new();
        assert_eq!(params.next_n(1), "$1");
    }

    #[test]
    fn next_n_twice() {
        let mut params = Parameters::new();
        params.next_n(3);
        assert_eq!(params.next_n(3), "$4, $5, $6");
    }

    #[test]
    fn next_arr() {
        let mut params = Parameters::new();
        let p = params.next_array::<5>();
        assert_eq!(p, ["$1", "$2", "$3", "$4", "$5"]);
    }

    #[test]
    fn next_arr_twice() {
        let mut params = Parameters::new();
        let p1 = params.next_array::<2>();
        let p2 = params.next_array::<3>();
        assert_eq!(p1, ["$1", "$2"]);
        assert_eq!(p2, ["$3", "$4", "$5"]);
    }
}
