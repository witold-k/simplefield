// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use struct_extractors::extract_accessors;
use lineariterator::niterator::NMutIterator;
use crate::orientation::{ColumnMajor, Orientation, RowMajor};

/// An owned, multi-dimensional data field with compile-time layout orientation.
///
/// `Field` allocates elements on the heap via a contiguous vector, mapping a 2D multi-index
/// space onto linear storage based on the layout strategy type parameter `O`.
///
/// ### Dimension Concept
/// Memory mapping relies on a decoupled dimension hierarchy:
/// * **`dim0_size` (Inner Dimension):** The contiguous, fast-changing axis. Elements along this axis sit side-by-side in raw memory (pointer stride = 1).
/// * **`dim1_size` (Outer Dimension):** The strided, slow-changing axis. Stepping through this axis requires jumping over segments of the inner dimension.
#[extract_accessors]
#[derive(Debug, Clone, Hash)]
pub struct Field<O, T>
where
    O: Orientation
{
    /// The size of the inner, fast-changing dimension in memory.
    #[access(get)]
    dim0_size: usize,

    /// The size of the outer, slow-changing dimension in memory.
    #[access(get)]
    dim1_size: usize,

    /// Contiguous heap allocation storing all multidimensional elements.
    #[access(get_ref)]
    data: Vec<T>,

    /// Compile-time phantom marker dictating layout math without runtime overhead.
    _marker: std::marker::PhantomData<O>
}

impl<O, T> Default for Field<O, T>
where
    O: Orientation,
    T: Clone
{
    /// Creates an empty field with zero dimensions and no memory allocations.
    fn default() -> Self {
        Self {
            dim0_size: 0,
            dim1_size: 0,
            data: Vec::new(),
            _marker: std::marker::PhantomData
        }
    }
}

impl<O, T> Field<O, T>
where
    O: Orientation,
    T: Clone,
{
    /// Returns an immutable reference to the underlying linear vector allocation.
    #[inline(always)]
    pub fn get_data(&self) -> &Vec<T> {
        &self.data
    }
}

// ===========================================================================
// IMPLEMENTATION FOR ROW-MAJOR (dim0 = columns, dim1 = rows)
// ===========================================================================
impl<T> Field<RowMajor, T>
where
    T: Clone,
{
    /// Creates an uninitialized row-major field reserving heap capacity for the specified grid size.
    pub fn new(row_count: usize, column_count: usize) -> Self {
        Self {
            dim0_size: column_count, // cols are fast-changing
            dim1_size: row_count,    // rows are slow-changing
            data: Vec::with_capacity(row_count * column_count),
            _marker: std::marker::PhantomData
        }
    }

    /// Creates a row-major field where every coordinate is filled with clones of the provided element.
    pub fn new_fill(row_count: usize, column_count: usize, data: T) -> Self {
        let s = row_count * column_count;
        let mut v = Vec::with_capacity(s);
        v.resize(s, data);
        Self {
            dim0_size: column_count,
            dim1_size: row_count,
            data: v,
            _marker: std::marker::PhantomData
        }
    }

    /// Wraps an existing flat linear vector into a row-major layout configuration.
    pub fn new_data(row_count: usize, column_count: usize, data: Vec<T>) -> Self {
        Self {
            dim0_size: column_count,
            dim1_size: row_count,
            data,
            _marker: std::marker::PhantomData
        }
    }

    /// Overwrites a subsection of an explicit row beginning at a specified column offset.
    ///
    /// # Safety
    /// This method is unsafe as it performs unchecked direct pointer arithmetic to write values into memory.
    /// Users must guarantee that the target range fits inside the field's allocated row boundaries.
    pub fn set_row_cut(&mut self, row: usize, column: usize, new_row: &[T]) {
        unsafe {
            let start = self.data.as_mut_ptr().add(column + row * self.dim0_size);
            let mut i = NMutIterator::new(start, self.dim0_size - column);
            i.clone_from_slice(new_row);
        }
    }

    /// Returns a zero-overhead, mutable linear iterator traversing the specified row index.
    ///
    /// Since rows are contiguous in row-major layout, this iterator sweeps sequentially with a stride of 1.
    ///
    /// # Safety
    /// Relies on unchecked pointer calculations to find the row head.
    pub fn row_mut_iterator(&mut self, row: usize) -> NMutIterator<'_, T> {
        unsafe {
            let start = self.data.as_mut_ptr().add(row * self.dim0_size);
            NMutIterator::new(start, self.dim0_size)
        }
    }

    /// Returns a zero-overhead, mutable strided iterator traversing the specified column index.
    ///
    /// Since column values are scattered in row-major layouts, this iterator hops with a step size equal to `dim0_size`.
    ///
    /// # Safety
    /// Relies on unchecked pointer calculations to find the column head and execute strides.
    pub fn column_mut_iterator(&mut self, column: usize) -> NMutIterator<'_, T> {
        unsafe {
            let start = self.data.as_mut_ptr().add(column);
            NMutIterator::new_step(start, self.dim1_size, self.dim0_size)
        }
    }
}

// ===========================================================================
// IMPLEMENTATION FOR COLUMN-MAJOR (dim0 = rows, dim1 = columns)
// ===========================================================================
impl<T> Field<ColumnMajor, T>
where
    T: Clone,
{
    /// Creates an uninitialized column-major field reserving heap capacity for the specified grid size.
    pub fn new(row_count: usize, column_count: usize) -> Self {
        Self {
            dim0_size: row_count,    // rows are fast-changing
            dim1_size: column_count, // cols are slow-changing
            data: Vec::with_capacity(row_count * column_count),
            _marker: std::marker::PhantomData,
        }
    }

    /// Creates a column-major field where every coordinate is filled with clones of the provided element.
    pub fn new_fill(row_count: usize, column_count: usize, data: T) -> Self {
        let s = row_count * column_count;
        let mut v = Vec::with_capacity(s);
        v.resize(s, data);
        Self {
            dim0_size: row_count,
            dim1_size: column_count,
            data: v,
            _marker: std::marker::PhantomData,
        }
    }

    /// Wraps an existing flat linear vector into a column-major layout configuration.
    pub fn new_data(row_count: usize, column_count: usize, data: Vec<T>) -> Self {
        Self {
            dim0_size: row_count,
            dim1_size: column_count,
            data,
            _marker: std::marker::PhantomData,
        }
    }

    /// Overwrites a subsection of an explicit row beginning at a specified column offset.
    ///
    /// # Safety
    /// This method is unsafe as it performs unchecked strided pointer writes. Rows are not contiguous
    /// in column-major configurations, so this operates using step logic.
    pub fn set_row_cut(&mut self, row: usize, column: usize, new_row: &[T]) {
        unsafe {
            let start = self.data.as_mut_ptr().add(row + column * self.dim0_size);
            let mut i = NMutIterator::new_step(start, self.dim1_size - column, self.dim0_size);
            i.clone_from_slice(new_row);
        }
    }

    /// Returns a zero-overhead, mutable strided iterator traversing the specified row index.
    ///
    /// Since rows are scattered across memory in column-major layouts, this iterator hops with a step size equal to `dim0_size`.
    ///
    /// # Safety
    /// Relies on unchecked pointer calculations to find the row head and execute strides.
    pub fn row_mut_iterator(&mut self, row: usize) -> NMutIterator<'_, T> {
        unsafe {
            let start = self.data.as_mut_ptr().add(row);
            NMutIterator::new_step(start, self.dim1_size, self.dim0_size)
        }
    }

    /// Returns a zero-overhead, mutable linear iterator traversing the specified column index.
    ///
    /// Since columns are contiguous in column-major layout, this iterator sweeps sequentially with a stride of 1.
    ///
    /// # Safety
    /// Relies on unchecked pointer calculations to find the column head.
    pub fn column_mut_iterator(&mut self, column: usize) -> NMutIterator<'_, T> {
        unsafe {
            let start = self.data.as_mut_ptr().add(column * self.dim0_size);
            NMutIterator::new(start, self.dim0_size)
        }
    }
}

