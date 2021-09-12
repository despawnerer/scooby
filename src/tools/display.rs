use std::fmt::{self, Display, Formatter};

pub fn joined<'a, I, T>(iter: I, sep: &'a str) -> Joined<'a, I::IntoIter, T>
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

        match iter.next() {
            Some(first) => {
                first.fmt(f)?;
                for item in iter {
                    f.write_str(self.sep)?;
                    item.fmt(f)?;
                }
            }
            _ => {}
        }

        Ok(())
    }
}
