// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use struct_extractors::extract_accessors;
use crate::orientation::{Orientation};

#[extract_accessors]
#[derive(Debug, Clone, Hash)]
pub struct RefField<'a, O, T>
where
    O: Orientation
{
    #[access(get)]
    orientation:  O,
    #[access(get)]
    row_count:    usize,
    #[access(get)]
    column_count: usize,
    #[access(get)]
    data:         &'a Vec<T>
}

// ---------------------------------------------------------------------------

impl<'a, O, T> RefField<'a, O, T>
where
    O: Orientation
{
    pub fn new(
        orientation:  O,
        row_count:    usize,
        column_count: usize,
        data:         &'a Vec::<T>
    ) -> Self {
        Self {
            orientation,
            row_count,
            column_count,
            data
        }
    }

}
