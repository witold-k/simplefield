// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::orientation::Orientation;
use struct_extractors::extract_accessors;

/// A borrowed two-dimensional view with compile-time layout orientation.
#[extract_accessors]
#[derive(Debug, Clone, Hash)]
pub struct RefField<'a, O, T>
where
    O: Orientation,
{
    #[access(get)]
    row_count: usize,
    #[access(get)]
    column_count: usize,
    #[access(get)]
    data: &'a [T],
    _marker: std::marker::PhantomData<O>,
}

impl<'a, O, T> RefField<'a, O, T>
where
    O: Orientation,
{
    /// Creates a borrowed field view.
    ///
    /// # Panics
    ///
    /// Panics if the dimensions overflow `usize` or if `data.len()` is not
    /// exactly `row_count * column_count`.
    pub fn new(row_count: usize, column_count: usize, data: &'a [T]) -> Self {
        let expected = row_count
            .checked_mul(column_count)
            .expect("field dimensions overflow usize");
        assert_eq!(
            data.len(),
            expected,
            "data length does not match field dimensions"
        );

        Self {
            row_count,
            column_count,
            data,
            _marker: std::marker::PhantomData,
        }
    }
}
