use std::fmt::{self, Display, Formatter};

pub fn joined<I, T>(iter: I, sep: &str) -> Joined<'_, I::IntoIter, T>
where
    I: IntoIterator<Item = T>,
    I::IntoIter: Clone,
    T: Display,
{
    Joined {
        iter: iter.into_iter(),
        sep,
    }
}

pub struct Joined<'a, I, T>
where
    I: Iterator<Item = T> + Clone,
    T: Display,
{
    iter: I,
    sep: &'a str,
}

impl<'a, I, T> Display for Joined<'a, I, T>
where
    I: Iterator<Item = T> + Clone,
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut iter = self.iter.clone();

        if let Some(first) = iter.next() {
            first.fmt(f)?;
            for item in iter {
                f.write_str(self.sep)?;
                item.fmt(f)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::iter::{once, empty, successors};
    use super::*;

    #[test]
    fn no_items() {
        let iter = empty::<usize>();
        assert_eq!(joined(iter, ", ").to_string(), "");
    }

    #[test]
    fn one_item() {
        let iter = once("asdf");
        assert_eq!(joined(iter, ", ").to_string(), "asdf");
    }

    #[test]
    fn two_items() {
        let iter = successors(Some(0), |x| Some(x + 1)).take(2);
        assert_eq!(joined(iter, ", ").to_string(), "0, 1");
    }

    #[test]
    fn a_few_items() {
        let iter = successors(Some(0), |x| Some(x + 1)).take(10);
        assert_eq!(joined(iter, ", ").to_string(), "0, 1, 2, 3, 4, 5, 6, 7, 8, 9");
    }
}
