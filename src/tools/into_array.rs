pub trait IntoArray<T, const N: usize> {
    fn into_array(self) -> [T; N];
}

// Strings

impl IntoArray<String, 1> for String {
    fn into_array(self) -> [String; 1] {
        [self]
    }
}

impl IntoArray<String, 1> for &str {
    fn into_array(self) -> [String; 1] {
        [self.to_owned()]
    }
}

// Generic tuples

impl<T> IntoArray<T, 0> for () {
    fn into_array(self) -> [T; 0] {
        []
    }
}

impl<T, U> IntoArray<T, 1> for (U,)
where
    U: Into<T>,
{
    fn into_array(self) -> [T; 1] {
        [self.0.into()]
    }
}

impl<T, U1, U2> IntoArray<T, 2> for (U1, U2)
where
    U1: Into<T>,
    U2: Into<T>,
{
    fn into_array(self) -> [T; 2] {
        [self.0.into(), self.1.into()]
    }
}

impl<T, U1, U2, U3> IntoArray<T, 3> for (U1, U2, U3)
where
    U1: Into<T>,
    U2: Into<T>,
    U3: Into<T>,
{
    fn into_array(self) -> [T; 3] {
        [self.0.into(), self.1.into(), self.2.into()]
    }
}
