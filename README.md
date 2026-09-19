# simplefield

A small Rust library for two-dimensional contiguous storage with statically selected row-major or column-major layout.

`Field<O, T>` owns its data. `RefField<'a, O, T>` provides a borrowed view over an existing slice. Layout is represented by the zero-sized marker types `RowMajor` and `ColumnMajor`, so row/column traversal can select the appropriate contiguous or strided iterator without runtime layout dispatch.

## Layout

The storage model uses two physical dimensions:

- `dim0_size`: contiguous, fast-changing dimension.
- `dim1_size`: strided, slow-changing dimension.

For row-major storage, `dim0` is the column count and `dim1` is the row count:

```text
row 0: [C0 C1 C2 C3]
row 1: [C0 C1 C2 C3]

index = row * column_count + column
```

For column-major storage, `dim0` is the row count and `dim1` is the column count:

```text
column 0: [R0 R1 R2]
column 1: [R0 R1 R2]

index = column * row_count + row
```

A `Field` always contains exactly `row_count * column_count` initialized elements. Constructors reject inconsistent shapes and dimension overflow.

## Usage

```rust
use simplefield::field::Field;
use simplefield::orientation::{ColumnMajor, RowMajor};

let mut row_major = Field::<RowMajor, f32>::new_fill(3, 4, 0.0);
let column_major = Field::<ColumnMajor, f32>::new_fill(3, 4, 0.0);

row_major.set_row_cut(1, 1, &[1.0, 2.0]);

for value in &mut row_major.column_mut_iterator(1) {
    // NMutIterator exposes raw pointers; dereferencing them is explicit.
    unsafe { *value += 1.0 };
}

assert_eq!(row_major.get_data().len(), 12);
assert_eq!(column_major.get_data().len(), 12);
```

The mutable row and column iterators are provided by
[`lineariterator`](https://github.com/witold-k/lineariterator). Depending on the selected layout, traversal is either contiguous or uses the corresponding row/column stride.

## Borrowed views

`RefField` accepts any slice with the exact size implied by its dimensions:

```rust
use simplefield::orientation::ColumnMajor;
use simplefield::reffield::RefField;

let data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
let view = RefField::<ColumnMajor, _>::new(3, 2, &data);

assert_eq!(view.get_row_count(), 3);
assert_eq!(view.get_column_count(), 2);
assert_eq!(view.get_data(), &data);
```

## Design

- Row-major and column-major layout are selected through compile-time marker types.
- Storage is a single contiguous `Vec<T>`.
- Public constructors maintain the field shape invariant.
- Bounds are checked before internal raw-pointer arithmetic.
- Low-level strided traversal is delegated to `lineariterator`.
- The library intentionally stays small rather than providing a general ndarray/tensor API.

## Testing

Tests live under `tests/` and use the source filename with a `_test.rs` suffix where applicable. Run the project checks with:

```text
just build
```
