// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

//! Memory layout orientations for multi-dimensional data fields.
//!
//! This module provides the compile-time type markers (`RowMajor`, `ColumnMajor`)
//! and the run-time representations (`OrientationEnum`) used to define how multi-dimensional
//! structures map their multi-index coordinates onto linear memory arrays.

/// Runtime representation of data field memory layouts.
///
/// Useful for serialization, runtime reflection, or conditional branching when
/// static type information is erased.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrientationEnum {
    /// Data elements of the same row are stored contiguously in memory.
    RowMajor,
    /// Data elements of the same column are stored contiguously in memory.
    ColumnMajor,
}

/// Marker trait for compile-time matrix layout dispatching.
///
/// Implemented exclusively by zero-sized types (`RowMajor`, `ColumnMajor`) to enable
/// zero-cost, type-safe optimizations. It enforces that any structural layout layout
/// parameter is safely duplicable, cloneable, and printable during debugging passes.
pub trait Orientation: Clone + Copy + std::fmt::Debug {}

/// Type marker specifying C-style row-major memory layouts.
///
/// Elements sharing the same row index occupy contiguous blocks in raw memory.
/// Consequently, iterating across columns within a fixed row changes the memory pointer linearly
/// (stride = 1), yielding maximum hardware cache locality during sequential row iterations.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct RowMajor;

/// Type marker specifying Fortran-style column-major memory layouts.
///
/// Elements sharing the same column index occupy contiguous blocks in raw memory.
/// Consequently, iterating across rows within a fixed column changes the memory pointer linearly
/// (stride = 1), yielding maximum hardware cache locality during sequential column iterations.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ColumnMajor;

impl Orientation for RowMajor {}
impl Orientation for ColumnMajor {}

