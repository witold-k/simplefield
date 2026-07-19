# LinearField

A zero-cost, type-safe two-dimensional array and slice framework written in Rust.
It utilizes the **Type-State Pattern** and **Zero-Sized Types (ZSTs)** to enforce memory layout properties at compile time,
eliminating runtime dispatch overhead while guaranteeing optimal CPU cache locality.

## Features

* **Zero-Cost Abstractions:** Orientation strategies (`RowMajor` / `ColumnMajor`) compile down to zero bytes and completely disappear in production builds.
* **No Runtime Branching:** Pointer arithmetic, strides, and memory jumps are evaluated at compile time rather than relying on runtime `if/else` checks.
* **Blazing Fast Iterators:** Features zero-overhead, highly optimized `unsafe` step iterators (`NMutIterator`) tailored to the field's underlying layout.
* **Unified Ownership Models:** Supports fully owned heap layouts (`Field<O, T>`) as well as temporary, lightweight structural references (`RefField<'a, O, T>`).

---

## Architectural Overview

The core architecture splits the structural memory layout (how multidimensional indexes correspond to physical memory indices) into two layers:

1. **Inner Dimension (`dim0_size`):** The contiguous, fast-changing dimension. Moving along this axis changes the raw memory pointer by a stride of 1.
2. **Outer Dimension (`dim1_size`):** The strided, slow-changing dimension. Moving along this axis skips entire blocks of the inner dimension.

By abstracting memory layout via traits, the architecture maps rows and columns cleanly based on the chosen strategy:

```text
<--:----------- Linear Memory Sequence -----------─--->

 [Row-Major Layout]  -> dim0 = Column Index (Fast)  │  dim1 = Row Index (Slow)
   Index Calculation: [ (row * dim0_size) + column ]
   ┌-─ Row 0 -─┐┌-─ Row 1 -─┐
   [C0, C1, C2, C3][C0, C1, C2, C3]...

 [Column-Major Layout] -> dim0 = Row Index (Fast)   │  dim1 = Column Index (Slow)
   Index Calculation: [ (column * dim0_size) + row ]
   ┌- Column 0 ─┐┌- Column 1 ─┐
   [R0, R1, R2, R3][R0, R1, R2, R3]...
```

### Module Layout & Core Type Relations

```text
           ┌------------------─┐
           │ trait Orientation │◀------------------------┐
           └------------------─┘                         │
                    ▲                                    │
            ┌-------┴---------┐       (Binds statically) │
            │                 │                          │
 ┌-----------------┐ ┌--------------------┐              │
 │ struct RowMajor │ │ struct ColumnMajor │              │
 └-----------------┘ └--------------------┘              │
            ▲                 ▲                          │
            └-------┬---------┘                          │
                    │ (Determines Layout Strategy)       │
                    ▼                                    │
 ┌----------------------------------------------------┐  │
 │ struct Field<O: Orientation, T>                    │--┘
 ├----------------------------------------------------┤
 │ - dim0_size: usize  (Contiguous / Inner Dimension) │
 │ - dim1_size: usize  (Strided    / Outer Dimension) │
 │ - data: Vec<T>                                     │
 └----------------------------------------------------┘
```

---

## 🛠️ Components

### 1. The `orientation` Module
Defines compile-time structural markers. It uses `OrientationEnum` if your application requires runtime reflection or serialization boundaries.

```rust
pub trait Orientation: Clone + Copy + std::fmt::Debug {}
pub struct RowMajor;    // C-Style
pub struct ColumnMajor; // Fortran-Style
```

### 2. The `field` Module (`Field<O, T>`)
The owned container managing heap allocation (`Vec<T>`). Implementation methods are isolated using Rust's trait-specialization system to present different initializer arguments and step calculations based on structural demands.

### 3. The `reffield` Module (`RefField<'a, O, T>`)
A structural window over an external data vector. It allows borrowing massive data sets while maintaining zero-overhead, layout-safe accessors.

---

## Quick Start & Usage Examples

### 1. Instantiating Fields with Different Layouts

```rust
use linearfield::field::Field;
use linearfield::orientation::{RowMajor, ColumnMajor};

// Instantiate a RowMajor Matrix (3 Rows, 4 Columns)
// Contiguous index is the column index
let mut row_field: Field<RowMajor, f32> = Field::new_fill(3, 4, 0.0);

// Instantiate a ColumnMajor Matrix (3 Rows, 4 Columns)
// Contiguous index is the row index
let mut col_field: Field<ColumnMajor, f32> = Field::new_fill(3, 4, 0.0);
```

### 2. Working with Layout-Aware Iterators

Thanks to the static type dispatching system, requesting a column or row iterator runs highly specialized pointer algorithms completely stripped of conditional branches:

```rust
use linearfield::field::Field;
use linearfield::orientation::RowMajor;

let mut field = Field::new_fill(5, 5, 1.0);

// Iterating a row on a RowMajor matrix runs sequentially (Stride = 1)
let row_iter = field.row_mut_iterator(2);

// Iterating a column on a RowMajor matrix automatically steps dynamically (Stride = dim0_size)
let col_iter = field.column_mut_iterator(1);
```

### 3. Creating Zero-Copy Reference Views

If you receive flat linear vectors from network sockets, files, or external C-FFI boundaries, wrap them dynamically into a `RefField`:

```rust
use linearfield::reffield::RefField;
use linearfield::orientation::ColumnMajor;

let raw_data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

// Map data as a 4x2 Column-Major viewing window safely
let view = RefField::new(ColumnMajor, 4, 2, &raw_data);

println!("Inner Dimension (Rows): {}", view.get_row_count());
```

---

## Performance Guarantees

* **Monomorphization:** The Rust compiler clones the machine instructions for every variation of `Field<O, T>`. Your assembly instructions will directly load hardcoded calculations
    rather than evaluating layout logic fields repeatedly inside your performance loops.
* **Vectorization-Friendly:** Contiguous memory sweeps (such as column iteration in `ColumnMajor` or row iteration in `RowMajor`)
    are cleanly structured to enable automatic SIMD optimizations by LLVM.

