// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::orientation::{ColumnMajor, Orientation, RowMajor};
use lineariterator::niterator::NMutIterator;
use struct_extractors::extract_accessors;

/// An owned two-dimensional data field with compile-time layout orientation.
///
/// `Field` stores exactly `row_count * column_count` initialized elements in a
/// contiguous vector. `RowMajor` and `ColumnMajor` select how logical rows and
/// columns map onto that storage.
#[extract_accessors]
#[derive(Debug, Clone, Hash)]
pub struct Field<O, T>
where
    O: Orientation,
{
    /// Size of the contiguous, fast-changing dimension.
    #[access(get)]
    dim0_size: usize,

    /// Size of the strided, slow-changing dimension.
    #[access(get)]
    dim1_size: usize,

    /// Contiguous storage for all field elements.
    data: Vec<T>,

    _marker: std::marker::PhantomData<O>,
}

impl<O, T> Default for Field<O, T>
where
    O: Orientation,
{
    fn default() -> Self {
        Self {
            dim0_size: 0,
            dim1_size: 0,
            data: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<O, T> Field<O, T>
where
    O: Orientation,
{
    /// Returns the underlying elements as a slice.
    #[inline]
    pub fn get_data(&self) -> &[T] {
        &self.data
    }
}

impl<T> Field<RowMajor, T> {
    /// Returns the logical number of rows.
    #[inline]
    pub const fn row_count(&self) -> usize {
        self.dim1_size
    }

    /// Returns the logical number of columns.
    #[inline]
    pub const fn column_count(&self) -> usize {
        self.dim0_size
    }
}

impl<T> Field<ColumnMajor, T> {
    /// Returns the logical number of rows.
    #[inline]
    pub const fn row_count(&self) -> usize {
        self.dim0_size
    }

    /// Returns the logical number of columns.
    #[inline]
    pub const fn column_count(&self) -> usize {
        self.dim1_size
    }
}

fn element_count(row_count: usize, column_count: usize) -> usize {
    row_count
        .checked_mul(column_count)
        .expect("field dimensions overflow usize")
}

fn assert_data_len<T>(row_count: usize, column_count: usize, data: &[T]) {
    let expected = element_count(row_count, column_count);
    assert_eq!(
        data.len(),
        expected,
        "data length does not match field dimensions"
    );
}

// ===========================================================================
// ROW-MAJOR (dim0 = columns, dim1 = rows)
// ===========================================================================
impl<T> Field<RowMajor, T> {
    /// Creates a row-major field filled with clones of `value`.
    pub fn new_fill(row_count: usize, column_count: usize, value: T) -> Self
    where
        T: Clone,
    {
        let size = element_count(row_count, column_count);
        Self {
            dim0_size: column_count,
            dim1_size: row_count,
            data: vec![value; size],
            _marker: std::marker::PhantomData,
        }
    }

    /// Wraps an existing flat vector in row-major layout.
    ///
    /// # Panics
    ///
    /// Panics if the dimensions overflow `usize` or if `data.len()` is not
    /// exactly `row_count * column_count`.
    pub fn new_data(row_count: usize, column_count: usize, data: Vec<T>) -> Self {
        assert_data_len(row_count, column_count, &data);
        Self {
            dim0_size: column_count,
            dim1_size: row_count,
            data,
            _marker: std::marker::PhantomData,
        }
    }

    /// Replaces part of a row starting at `column`.
    ///
    /// # Panics
    ///
    /// Panics if `row` is out of bounds, `column` is past the end of the row,
    /// or `new_row` does not fit in the remaining columns.
    pub fn set_row_cut(&mut self, row: usize, column: usize, new_row: &[T])
    where
        T: Clone,
    {
        assert!(row < self.dim1_size, "row index out of bounds");
        assert!(column <= self.dim0_size, "column index out of bounds");
        assert!(
            new_row.len() <= self.dim0_size - column,
            "row data exceeds remaining columns"
        );

        if new_row.is_empty() {
            return;
        }

        unsafe {
            let start = self
                .data
                .as_mut_ptr()
                .add(row * self.dim0_size + column);
            let mut iter = NMutIterator::new(start, new_row.len());
            iter.clone_from_slice(new_row);
        }
    }

    /// Returns a mutable raw-pointer iterator over a row.
    ///
    /// # Panics
    ///
    /// Panics if `row` is out of bounds.
    pub fn row_mut_iterator(&mut self, row: usize) -> NMutIterator<'_, T> {
        assert!(row < self.dim1_size, "row index out of bounds");

        let start = if self.dim0_size == 0 {
            self.data.as_mut_ptr()
        } else {
            unsafe { self.data.as_mut_ptr().add(row * self.dim0_size) }
        };

        unsafe { NMutIterator::new(start, self.dim0_size) }
    }

    /// Returns a mutable raw-pointer iterator over a column.
    ///
    /// # Panics
    ///
    /// Panics if `column` is out of bounds.
    pub fn column_mut_iterator(&mut self, column: usize) -> NMutIterator<'_, T> {
        assert!(column < self.dim0_size, "column index out of bounds");

        let start = if self.dim1_size == 0 {
            self.data.as_mut_ptr()
        } else {
            unsafe { self.data.as_mut_ptr().add(column) }
        };

        unsafe { NMutIterator::new_step(start, self.dim1_size, self.dim0_size) }
    }
}

// ===========================================================================
// COLUMN-MAJOR (dim0 = rows, dim1 = columns)
// ===========================================================================
impl<T> Field<ColumnMajor, T> {
    /// Creates a column-major field filled with clones of `value`.
    pub fn new_fill(row_count: usize, column_count: usize, value: T) -> Self
    where
        T: Clone,
    {
        let size = element_count(row_count, column_count);
        Self {
            dim0_size: row_count,
            dim1_size: column_count,
            data: vec![value; size],
            _marker: std::marker::PhantomData,
        }
    }

    /// Wraps an existing flat vector in column-major layout.
    ///
    /// # Panics
    ///
    /// Panics if the dimensions overflow `usize` or if `data.len()` is not
    /// exactly `row_count * column_count`.
    pub fn new_data(row_count: usize, column_count: usize, data: Vec<T>) -> Self {
        assert_data_len(row_count, column_count, &data);
        Self {
            dim0_size: row_count,
            dim1_size: column_count,
            data,
            _marker: std::marker::PhantomData,
        }
    }

    /// Replaces part of a row starting at `column`.
    ///
    /// # Panics
    ///
    /// Panics if `row` is out of bounds, `column` is past the end of the row,
    /// or `new_row` does not fit in the remaining columns.
    pub fn set_row_cut(&mut self, row: usize, column: usize, new_row: &[T])
    where
        T: Clone,
    {
        assert!(row < self.dim0_size, "row index out of bounds");
        assert!(column <= self.dim1_size, "column index out of bounds");
        assert!(
            new_row.len() <= self.dim1_size - column,
            "row data exceeds remaining columns"
        );

        if new_row.is_empty() {
            return;
        }

        unsafe {
            let start = self
                .data
                .as_mut_ptr()
                .add(row + column * self.dim0_size);
            let mut iter = NMutIterator::new_step(start, new_row.len(), self.dim0_size);
            iter.clone_from_slice(new_row);
        }
    }

    /// Returns a mutable raw-pointer iterator over a row.
    ///
    /// # Panics
    ///
    /// Panics if `row` is out of bounds.
    pub fn row_mut_iterator(&mut self, row: usize) -> NMutIterator<'_, T> {
        assert!(row < self.dim0_size, "row index out of bounds");

        let start = if self.dim1_size == 0 {
            self.data.as_mut_ptr()
        } else {
            unsafe { self.data.as_mut_ptr().add(row) }
        };

        unsafe { NMutIterator::new_step(start, self.dim1_size, self.dim0_size) }
    }

    /// Returns a mutable raw-pointer iterator over a column.
    ///
    /// # Panics
    ///
    /// Panics if `column` is out of bounds.
    pub fn column_mut_iterator(&mut self, column: usize) -> NMutIterator<'_, T> {
        assert!(column < self.dim1_size, "column index out of bounds");

        let start = if self.dim0_size == 0 {
            self.data.as_mut_ptr()
        } else {
            unsafe { self.data.as_mut_ptr().add(column * self.dim0_size) }
        };

        unsafe { NMutIterator::new(start, self.dim0_size) }
    }
}
