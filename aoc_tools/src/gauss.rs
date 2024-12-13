use num::Rational64;

pub fn gauss_eliminate<const NROWS: usize, const NCOLS: usize> (m: &mut [[Rational64; NCOLS]; NROWS]) -> bool {
    for i in 0..NROWS {
        let d = m[i][i];
        if d == Rational64::ZERO {
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

        let mut matrix: [[Rational64; 3]; 2] = [
            [94.into(), 22.into(), 8400.into()],
            [34.into(), 67.into(), 5400.into()],
        ];

        let success = gauss_eliminate(&mut matrix);

        // let's round to eliminate floating-point imprecisions,
        // as we know that the expected result is whole numbers
        matrix[0][2] = matrix[0][2].round();
        matrix[1][2] = matrix[1][2].round();

        assert!(success);
        assert_eq!(matrix, [
            [1.into(), 0.into(), 80.into()],
            [0.into(), 1.into(), 40.into()],
        ])
    }
}