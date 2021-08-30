use std::fmt::{self, Display, Formatter};

use itertools::Itertools;

use crate::postgres::general::{Column, Expression};
use crate::tools::IntoNonZeroArray;

/// Marker trait for implemenations of different kinds of `VALUES`
/// clauses for `INSERT INTO` queries
///
/// You may not construct any of the implementations directly.
///
/// Please use the appropriate methods on [`BareInsertInto`][crate::postgres::queries::BareInsertInto]
pub trait Values: Display {}

/// Default values, i.e. `INSERT INTO x DEFAULT VALUES`
///
/// Constructing this directly is useless, please use [`BareInsertInto::default_values`][crate::postgres::queries::BareInsertInto::default_values]
#[derive(Debug)]
pub struct DefaultValues;

impl Values for DefaultValues {}

impl Display for DefaultValues {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "DEFAULT VALUES")
    }
}

/// Values without columns, i.e. `INSERT INTO x VALUES (1, 2)`
///
/// You may not construct this directly, please use [`BareInsertInto::values`][crate::postgres::queries::BareInsertInto::values]
#[derive(Debug)]
pub struct WithoutColumns<const N: usize> {
    values: Vec<[Expression; N]>,
}

impl<const N: usize> WithoutColumns<N> {
    pub(crate) fn new(values: Vec<[Expression; N]>) -> Self {
        WithoutColumns { values }
    }

    pub(crate) fn add<T: IntoNonZeroArray<Expression, N>>(
        &mut self,
        iter: impl IntoIterator<Item = T>,
    ) {
        self.values
            .extend(iter.into_iter().map(IntoNonZeroArray::into_non_zero_array))
    }
}

impl<const N: usize> Values for WithoutColumns<N> {}

impl<const N: usize> Display for WithoutColumns<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "VALUES {}",
            self.values
                .iter()
                .map(|cols| format!("({})", cols.iter().join(", ")))
                .join(", ")
        )
    }
}

/// Values with specified columns, i.e. `INSERT INTO x (col1, col2) VALUES (1, 2)`
///
/// You may not construct this directly, please use [`BareInsertInto::columns`][crate::postgres::queries::BareInsertInto::columns]
#[derive(Debug)]
pub struct WithColumns<const N: usize> {
    columns: [Column; N],
    values: Vec<[Expression; N]>,
}

impl<const N: usize> WithColumns<N> {
    pub(crate) fn new(columns: [Column; N], values: Vec<[Expression; N]>) -> Self {
        WithColumns { columns, values }
    }

    pub(crate) fn add<T: IntoNonZeroArray<Expression, N>>(
        &mut self,
        iter: impl IntoIterator<Item = T>,
    ) {
        self.values
            .extend(iter.into_iter().map(IntoNonZeroArray::into_non_zero_array))
    }
}

impl<const N: usize> Values for WithColumns<N> {}

impl<const N: usize> Display for WithColumns<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}) VALUES {}",
            self.columns.iter().join(", "),
            self.values
                .iter()
                .map(|cols| format!("({})", cols.iter().join(", ")))
                .join(", ")
        )
    }
}
