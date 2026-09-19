// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use simplefield::field::Field;
use simplefield::orientation::{ColumnMajor, RowMajor};
use simplefield::reffield::RefField;

#[test]
fn row_major_layout_and_iteration() {
    let mut field = Field::<RowMajor, i32>::new_data(3, 3, vec![
        1, 2, 3,
        4, 5, 6,
        7, 8, 9,
    ]);

    field.set_row_cut(1, 1, &[50, 60]);

    for value in &mut field.column_mut_iterator(0) {
        unsafe { *value *= 10 };
    }

    assert_eq!(field.get_data(), &[10, 2, 3, 40, 50, 60, 70, 8, 9]);
}

#[test]
fn column_major_layout_and_iteration() {
    let mut field = Field::<ColumnMajor, i32>::new_data(3, 3, vec![
        1, 2, 3,
        4, 5, 6,
        7, 8, 9,
    ]);

    field.set_row_cut(1, 1, &[50, 80]);

    for value in &mut field.column_mut_iterator(0) {
        unsafe { *value *= 10 };
    }

    assert_eq!(field.get_data(), &[10, 20, 30, 4, 50, 6, 7, 80, 9]);
}

#[test]
fn fill_creates_fully_initialized_storage() {
    let row = Field::<RowMajor, String>::new_fill(2, 3, String::from("x"));
    let column = Field::<ColumnMajor, String>::new_fill(2, 3, String::from("y"));

    assert_eq!(row.get_data().len(), 6);
    assert_eq!(column.get_data().len(), 6);
    assert!(row.get_data().iter().all(|value| value == "x"));
    assert!(column.get_data().iter().all(|value| value == "y"));
}

#[test]
#[should_panic(expected = "data length does not match field dimensions")]
fn row_major_rejects_wrong_data_length() {
    let _ = Field::<RowMajor, i32>::new_data(2, 3, vec![1, 2, 3]);
}

#[test]
#[should_panic(expected = "data length does not match field dimensions")]
fn column_major_rejects_wrong_data_length() {
    let _ = Field::<ColumnMajor, i32>::new_data(2, 3, vec![1, 2, 3]);
}

#[test]
#[should_panic(expected = "field dimensions overflow usize")]
fn dimensions_reject_overflow() {
    let _ = Field::<RowMajor, u8>::new_data(usize::MAX, 2, Vec::new());
}

#[test]
#[should_panic(expected = "row index out of bounds")]
fn row_iterator_rejects_invalid_row() {
    let mut field = Field::<RowMajor, i32>::new_fill(2, 3, 0);
    let _ = field.row_mut_iterator(2);
}

#[test]
#[should_panic(expected = "column index out of bounds")]
fn column_iterator_rejects_invalid_column() {
    let mut field = Field::<ColumnMajor, i32>::new_fill(2, 3, 0);
    let _ = field.column_mut_iterator(3);
}

#[test]
#[should_panic(expected = "row data exceeds remaining columns")]
fn row_cut_rejects_data_past_row_end() {
    let mut field = Field::<RowMajor, i32>::new_fill(2, 3, 0);
    field.set_row_cut(0, 2, &[1, 2]);
}

#[test]
fn empty_axes_produce_empty_iterators_without_pointer_offsets() {
    let mut no_rows = Field::<RowMajor, i32>::new_data(0, 3, vec![]);
    assert_eq!(no_rows.column_mut_iterator(2).count(), 0);

    let mut no_columns = Field::<ColumnMajor, i32>::new_data(3, 0, vec![]);
    assert_eq!(no_columns.row_mut_iterator(2).count(), 0);
}

#[test]
fn zero_sized_elements_preserve_logical_shape() {
    let mut field = Field::<RowMajor, ()>::new_fill(2, 3, ());
    assert_eq!(field.get_data().len(), 6);
    assert_eq!(field.row_mut_iterator(1).count(), 3);
    assert_eq!(field.column_mut_iterator(2).count(), 2);
}

#[test]
fn ref_field_accepts_slices_and_validates_shape() {
    let data = [1, 2, 3, 4, 5, 6];
    let field = RefField::<RowMajor, _>::new(2, 3, &data);

    assert_eq!(field.get_row_count(), 2);
    assert_eq!(field.get_column_count(), 3);
    assert_eq!(field.get_data(), data.as_slice());
}

#[test]
#[should_panic(expected = "data length does not match field dimensions")]
fn ref_field_rejects_wrong_shape() {
    let data = [1, 2, 3];
    let _ = RefField::<ColumnMajor, _>::new(2, 2, &data);
}

#[test]
fn logical_dimensions_are_independent_of_storage_layout() {
    let row = Field::<RowMajor, i32>::new_fill(2, 3, 0);
    let column = Field::<ColumnMajor, i32>::new_fill(2, 3, 0);

    assert_eq!((row.row_count(), row.column_count()), (2, 3));
    assert_eq!((column.row_count(), column.column_count()), (2, 3));
}

#[test]
fn default_is_an_empty_zero_by_zero_field() {
    let row = Field::<RowMajor, i32>::default();
    let column = Field::<ColumnMajor, i32>::default();

    assert_eq!((row.row_count(), row.column_count()), (0, 0));
    assert_eq!((column.row_count(), column.column_count()), (0, 0));
    assert!(row.get_data().is_empty());
    assert!(column.get_data().is_empty());
}
