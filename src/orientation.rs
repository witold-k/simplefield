// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

//! Compile-time memory-layout markers for two-dimensional fields.

/// Runtime representation of a field layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrientationEnum {
    RowMajor,
    ColumnMajor,
}

/// Marker trait for compile-time field layout selection.
pub trait Orientation: Clone + Copy + std::fmt::Debug {
    /// Runtime representation of this layout, for boundaries that need one.
    const ORIENTATION: OrientationEnum;
}

/// C-style row-major layout.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct RowMajor;

/// Fortran-style column-major layout.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ColumnMajor;

impl Orientation for RowMajor {
    const ORIENTATION: OrientationEnum = OrientationEnum::RowMajor;
}

impl Orientation for ColumnMajor {
    const ORIENTATION: OrientationEnum = OrientationEnum::ColumnMajor;
}
