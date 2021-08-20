use itertools::Itertools;

/// Generate PostgreSQL parameter placeholders for dynamic queries with multiple values
pub struct Parameters {
    current: usize,
}

impl Parameters {
    pub fn new() -> Parameters {
        Parameters { current: 1 }
    }

    pub fn next(&mut self) -> String {
        let s = format!("${}", self.current);
        self.current += 1;
        s
    }

    pub fn next_n(&mut self, n: usize) -> String {
        let last = self.current + n;
        let s = (self.current..last).map(|x| format!("${}", x)).join(", ");
        self.current = last;
        s
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
}
