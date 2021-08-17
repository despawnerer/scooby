pub trait IntoArrayOfSameType<T, const N: usize> {
    fn into_array(self) -> [T; N];
}

// Strings

impl IntoArrayOfSameType<String, 1> for String {
    fn into_array(self) -> [String; 1] {
        [self]
    }
}

impl IntoArrayOfSameType<String, 1> for &str {
    fn into_array(self) -> [String; 1] {
        [self.to_owned()]
    }
}

// Generic arrays

impl<T, const N: usize> IntoArrayOfSameType<T, N> for [T; N] {
    fn into_array(self) -> [T; N] {
        self
    }
}

// Generic tuples

impl<T> IntoArrayOfSameType<T, 0> for () {
    fn into_array(self) -> [T; 0] {
        []
    }
}

impl<T, U> IntoArrayOfSameType<T, 1> for (U,)
where
    U: Into<T>,
{
    fn into_array(self) -> [T; 1] {
        [self.0.into()]
    }
}

impl<T, U1, U2> IntoArrayOfSameType<T, 2> for (U1, U2)
where
    U1: Into<T>,
    U2: Into<T>,
{
    fn into_array(self) -> [T; 2] {
        [self.0.into(), self.1.into()]
    }
}

impl<T, U1, U2, U3, U4> IntoArrayOfSameType<T, 4> for (U1, U2, U3, U4)
where
    U1: Into<T>,
    U2: Into<T>,
    U3: Into<T>,
    U4: Into<T>,
{
    fn into_array(self) -> [T; 4] {
        [self.0.into(), self.1.into(), self.2.into(), self.3.into()]
    }
}

impl<T, U1, U2, U3, U4, U5> IntoArrayOfSameType<T, 5> for (U1, U2, U3, U4, U5)
where
    U1: Into<T>,
    U2: Into<T>,
    U3: Into<T>,
    U4: Into<T>,
    U5: Into<T>,
{
    fn into_array(self) -> [T; 5] {
        [
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
        ]
    }
}

impl<T, U1, U2, U3, U4, U5, U6> IntoArrayOfSameType<T, 6> for (U1, U2, U3, U4, U5, U6)
where
    U1: Into<T>,
    U2: Into<T>,
    U3: Into<T>,
    U4: Into<T>,
    U5: Into<T>,
    U6: Into<T>,
{
    fn into_array(self) -> [T; 6] {
        [
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
        ]
    }
}

impl<T, U1, U2, U3, U4, U5, U6, U7> IntoArrayOfSameType<T, 7> for (U1, U2, U3, U4, U5, U6, U7)
where
    U1: Into<T>,
    U2: Into<T>,
    U3: Into<T>,
    U4: Into<T>,
    U5: Into<T>,
    U6: Into<T>,
    U7: Into<T>,
{
    fn into_array(self) -> [T; 7] {
        [
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
        ]
    }
}

impl<T, U1, U2, U3, U4, U5, U6, U7, U8> IntoArrayOfSameType<T, 8> for (U1, U2, U3, U4, U5, U6, U7, U8)
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
    fn into_array(self) -> [T; 8] {
        [
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
        ]
    }
}

impl<T, U1, U2, U3, U4, U5, U6, U7, U8, U9> IntoArrayOfSameType<T, 9> for (U1, U2, U3, U4, U5, U6, U7, U8, U9)
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
    fn into_array(self) -> [T; 9] {
        [
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
        ]
    }
}
