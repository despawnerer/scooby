use std::iter::{empty, once, Chain, Copied, Empty, Map, Once};
use std::slice;

pub trait IntoSomeIterator<T> {
    type Iterator: Iterator<Item = T>;

    fn into_some_iter(self) -> Self::Iterator;
}

// Strings

impl<T> IntoSomeIterator<T> for String where T: From<String> {
    type Iterator = Once<T>;

    fn into_some_iter(self) -> Self::Iterator {
        once(self.into())
    }
}

impl<'a, T> IntoSomeIterator<T> for &'a str where T: From<&'a str> {
    type Iterator = Once<T>;

    fn into_some_iter(self) -> Self::Iterator {
        once(self.into())
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

// Generic arrays

impl<T, U, const N: usize> IntoSomeIterator<T> for [U; N] where U: Into<T> {
    type Iterator = Map<std::array::IntoIter<U, N>, fn(U) -> T>;

    fn into_some_iter(self) -> Self::Iterator {
        IntoIterator::into_iter(self).map(U::into)
    }
}

// Generic tuples
// TODO: A macro

impl<T> IntoSomeIterator<T> for () {
    type Iterator = Empty<T>;

    fn into_some_iter(self) -> Self::Iterator {
        empty()
    }
}

impl<T, U> IntoSomeIterator<T> for (U,) where U: Into<T> {
    type Iterator = Once<T>;

    fn into_some_iter(self) -> Self::Iterator {
        once(self.0.into())
    }
}

impl<T, U1, U2> IntoSomeIterator<T> for (U1, U2) where U1: Into<T>, U2: Into<T> {
    type Iterator = Chain<Once<T>, Once<T>>;

    fn into_some_iter(self) -> Self::Iterator {
        once(self.0.into()).chain(once(self.1.into()))
    }
}

impl<T, U1, U2, U3> IntoSomeIterator<T> for (U1, U2, U3) where U1: Into<T>, U2: Into<T>, U3: Into<T> {
    type Iterator = Chain<Chain<Once<T>, Once<T>>, Once<T>>;

    fn into_some_iter(self) -> Self::Iterator {
        once(self.0.into())
            .chain(once(self.1.into()))
            .chain(once(self.2.into()))
    }
}

impl<T, U1, U2, U3, U4, U5, U6, U7> IntoSomeIterator<T> for (U1, U2, U3, U4, U5, U6, U7)
    where
        U1: Into<T>,
        U2: Into<T>,
        U3: Into<T>,
        U4: Into<T>,
        U5: Into<T>,
        U6: Into<T>,
        U7: Into<T>,
{
    type Iterator = Chain<Chain<Chain<Chain<Chain<Chain<Once<T>, Once<T>>, Once<T>>, Once<T>>, Once<T>>, Once<T>>, Once<T>>;

    fn into_some_iter(self) -> Self::Iterator {
        once(self.0.into())
            .chain(once(self.1.into()))
            .chain(once(self.2.into()))
            .chain(once(self.3.into()))
            .chain(once(self.4.into()))
            .chain(once(self.5.into()))
            .chain(once(self.6.into()))
    }
}
