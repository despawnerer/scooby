use std::iter::{empty, once, repeat, Chain, Copied, Empty, Map, Once};
use std::slice;

use itertools::Itertools;

pub fn q(n: usize) -> String {
    repeat("?").take(n).join(", ")
}

pub trait IntoSomeIterator<T> {
    type Iterator: Iterator<Item = T>;

    fn into_some_iter(self) -> Self::Iterator;
}

impl IntoSomeIterator<String> for &str {
    type Iterator = Once<String>;

    fn into_some_iter(self) -> Self::Iterator {
        once(self.to_string())
    }
}

impl IntoSomeIterator<String> for () {
    type Iterator = Empty<String>;

    fn into_some_iter(self) -> Self::Iterator {
        empty()
    }
}

// TODO: A macro

impl<'a> IntoSomeIterator<String> for (&'a str,) {
    type Iterator = Once<String>;

    fn into_some_iter(self) -> Self::Iterator {
        once(self.0.to_string())
    }
}

impl<'a> IntoSomeIterator<String> for (&'a str, &'a str) {
    type Iterator = Chain<Once<String>, Once<String>>;

    fn into_some_iter(self) -> Self::Iterator {
        once(self.0.to_string()).chain(once(self.1.to_string()))
    }
}

impl<'a> IntoSomeIterator<String> for (&'a str, &'a str, &'a str) {
    type Iterator = Chain<Chain<Once<String>, Once<String>>, Once<String>>;

    fn into_some_iter(self) -> Self::Iterator {
        once(self.0.to_string())
            .chain(once(self.1.to_string()))
            .chain(once(self.2.to_string()))
    }
}

impl<'a, T> IntoSomeIterator<String> for &'a T
where
    T: AsRef<[&'a str]>,
{
    // Jesus fucking Christ almighty, there is no God.
    type Iterator = Map<Copied<slice::Iter<'a, &'a str>>, fn(&'a str) -> String>;

    fn into_some_iter(self) -> Self::Iterator {
        self.as_ref().iter().copied().map(str::to_string)
    }
}
