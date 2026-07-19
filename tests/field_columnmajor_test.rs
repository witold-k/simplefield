// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

#[cfg(test)]
mod tests {
    use simplefield::field::Field;
    use simplefield::orientation::ColumnMajor;


    #[test]
    fn test_new_column_field() {
        let f = Field::<ColumnMajor, i32>::new(3, 4);

        assert_eq!(f.get_dim1_size(), 4); // columns
        assert_eq!(f.get_dim0_size(), 3); // rows
        assert_eq!(f.get_data().len(), 0);
        assert_eq!(f.get_data().capacity(), 12);
    }

    #[test]
    fn test_set_row_cut() {
        let mut f = Field::<ColumnMajor, i32>::new_fill(3, 4, 0);

        // Replace row 1, starting at column 1
        f.set_row_cut(1, 1, &[10, 11, 12]);

        let expected = vec![
            0, 0,  0, // column 0
            0, 10, 0, // column 1
            0, 11, 0, // column 2
            0, 12, 0  // column 3
        ];

        assert_eq!(f.get_ref_data(), &expected);
    }

    #[test]
    fn test_row_mut_iterator() {
        let mut f = Field::<ColumnMajor, i32>::new_data(2, 3, vec![1, 2, 3, 4, 5, 6]);

        {
            let mut it = f.row_mut_iterator(1);
            for v in &mut it {
                unsafe { *v *= 10; }
            }
        }

        assert_eq!(f.get_ref_data(), &vec![1, 20, 3, 40, 5, 60]);
    }

    #[test]
    fn test_column_mut_iterator() {
        let mut f = Field::<ColumnMajor, i32>::new_data(3, 3, vec![
            1, 2, 3,
            4, 5, 6,
            7, 8, 9,
        ]);

        {
            let mut it = f.column_mut_iterator(1);
            for v in &mut it {
                unsafe { *v += 100; }
            }
        }

        let expected = vec![
            1, 2, 3,
            104, 105, 106,
            7, 8, 9,
        ];

        assert_eq!(f.get_ref_data(), &expected);
    }
}

