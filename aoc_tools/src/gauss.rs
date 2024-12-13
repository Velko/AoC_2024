pub fn gauss_eliminate<const NROWS: usize, const NCOLS: usize> (m: &mut [[f64; NCOLS]; NROWS]) -> bool {
    for i in 0..NROWS {
        let d = m[i][i];
        if d == 0.0 {
            return false;
        }

        for c in i..NCOLS {
            m[i][c] /= d;
        }

        for j in (i + 1)..NROWS {
            let mul = m[j][i];
            for c in i..NCOLS {
                m[j][c] -= m[i][c] * mul;
            }
        }
    }

    for i in (1..NROWS).rev() {
        for j in 0..i {
            let mul = m[j][i];
            for c in i..NCOLS {
                m[j][c] -= m[i][c] * mul;
            }
        }
    }

    true
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_rev_loop() {
        let nrows = 3;

        let l: Vec<_> = (1..nrows).rev().collect();

        assert_eq!(l, vec![2, 1]);
    }

    #[test]
    fn test_solve_sample() {

        let mut matrix: [[f64; 3]; 2] = [
            [94.0, 22.0, 8400.0],
            [34.0, 67.0, 5400.0],
        ];

        let success = gauss_eliminate(&mut matrix);

        // let's round to eliminate floating-point imprecisions,
        // as we know that the expected result is whole numbers
        matrix[0][2] = matrix[0][2].round();
        matrix[1][2] = matrix[1][2].round();

        assert!(success);
        assert_eq!(matrix, [
            [1.0, 0.0, 80.0],
            [0.0, 1.0, 40.0],
        ])
    }
}