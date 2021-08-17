use std::iter::{empty, once, Copied, Empty, Map, Once};
use std::slice;

pub trait IntoIteratorOfSameType<T> {
    type Iterator: Iterator<Item = T>;

    fn into_some_iter(self) -> Self::Iterator;
}

// Strings

impl<T> IntoIteratorOfSameType<T> for String
where
    T: From<String>,
{
    type Iterator = Once<T>;

    fn into_some_iter(self) -> Self::Iterator {
        once(self.into())
    }
}

impl<'a, T> IntoIteratorOfSameType<T> for &'a str
where
    T: From<&'a str>,
{
    type Iterator = Once<T>;

    fn into_some_iter(self) -> Self::Iterator {
        once(self.into())
    }
}

impl<'a, T> IntoIteratorOfSameType<String> for &'a T
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

impl<T, U, const N: usize> IntoIteratorOfSameType<T> for [U; N]
where
    U: Into<T>,
{
    type Iterator = Map<std::array::IntoIter<U, N>, fn(U) -> T>;

    fn into_some_iter(self) -> Self::Iterator {
        IntoIterator::into_iter(self).map(U::into)
    }
}

// Generic tuples
// TODO: A macro

impl<T> IntoIteratorOfSameType<T> for () {
    type Iterator = Empty<T>;

    fn into_some_iter(self) -> Self::Iterator {
        empty()
    }
}

impl<T, U> IntoIteratorOfSameType<T> for (U,)
where
    U: Into<T>,
{
    type Iterator = std::array::IntoIter<T, 1>;

    fn into_some_iter(self) -> Self::Iterator {
        IntoIterator::into_iter([self.0.into()])
    }
}

impl<T, U1, U2> IntoIteratorOfSameType<T> for (U1, U2)
where
    U1: Into<T>,
    U2: Into<T>,
{
    type Iterator = std::array::IntoIter<T, 2>;

    fn into_some_iter(self) -> Self::Iterator {
        IntoIterator::into_iter([self.0.into(), self.1.into()])
    }
}

impl<T, U1, U2, U3> IntoIteratorOfSameType<T> for (U1, U2, U3)
where
    U1: Into<T>,
    U2: Into<T>,
    U3: Into<T>,
{
    type Iterator = std::array::IntoIter<T, 3>;

    fn into_some_iter(self) -> Self::Iterator {
        IntoIterator::into_iter([self.0.into(), self.1.into(), self.2.into()])
    }
}

impl<T, U1, U2, U3, U4> IntoIteratorOfSameType<T> for (U1, U2, U3, U4)
where
    U1: Into<T>,
    U2: Into<T>,
    U3: Into<T>,
    U4: Into<T>,
{
    type Iterator = std::array::IntoIter<T, 4>;

    fn into_some_iter(self) -> Self::Iterator {
        IntoIterator::into_iter([self.0.into(), self.1.into(), self.2.into(), self.3.into()])
    }
}

impl<T, U1, U2, U3, U4, U5> IntoIteratorOfSameType<T> for (U1, U2, U3, U4, U5)
where
    U1: Into<T>,
    U2: Into<T>,
    U3: Into<T>,
    U4: Into<T>,
    U5: Into<T>,
{
    type Iterator = std::array::IntoIter<T, 5>;

    fn into_some_iter(self) -> Self::Iterator {
        IntoIterator::into_iter([
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
        ])
    }
}

impl<T, U1, U2, U3, U4, U5, U6> IntoIteratorOfSameType<T> for (U1, U2, U3, U4, U5, U6)
where
    U1: Into<T>,
    U2: Into<T>,
    U3: Into<T>,
    U4: Into<T>,
    U5: Into<T>,
    U6: Into<T>,
{
    type Iterator = std::array::IntoIter<T, 6>;

    fn into_some_iter(self) -> Self::Iterator {
        IntoIterator::into_iter([
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
        ])
    }
}

impl<T, U1, U2, U3, U4, U5, U6, U7> IntoIteratorOfSameType<T> for (U1, U2, U3, U4, U5, U6, U7)
where
    U1: Into<T>,
    U2: Into<T>,
    U3: Into<T>,
    U4: Into<T>,
    U5: Into<T>,
    U6: Into<T>,
    U7: Into<T>,
{
    type Iterator = std::array::IntoIter<T, 7>;

    fn into_some_iter(self) -> Self::Iterator {
        IntoIterator::into_iter([
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
        ])
    }
}

impl<T, U1, U2, U3, U4, U5, U6, U7, U8> IntoIteratorOfSameType<T> for (U1, U2, U3, U4, U5, U6, U7, U8)
where
    U1: Into<T>,
    U2: Into<T>,
    U3: Into<T>,
    U4: Into<T>,
    U5: Into<T>,
    U6: Into<T>,
    U7: Into<T>,
    U8: Into<T>,
{
    type Iterator = std::array::IntoIter<T, 8>;

    fn into_some_iter(self) -> Self::Iterator {
        IntoIterator::into_iter([
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
        ])
    }
}

impl<T, U1, U2, U3, U4, U5, U6, U7, U8, U9> IntoIteratorOfSameType<T>
    for (U1, U2, U3, U4, U5, U6, U7, U8, U9)
where
    U1: Into<T>,
    U2: Into<T>,
    U3: Into<T>,
    U4: Into<T>,
    U5: Into<T>,
    U6: Into<T>,
    U7: Into<T>,
    U8: Into<T>,
    U9: Into<T>,
{
    type Iterator = std::array::IntoIter<T, 9>;

    fn into_some_iter(self) -> Self::Iterator {
        IntoIterator::into_iter([
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
        ])
    }
}
