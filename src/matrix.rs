use crate::xor;

#[derive(Debug, PartialEq)]
pub(crate) struct ByteMatrix<const M: usize, const N: usize> {
    inner: [[u8; N]; M],
    rows: usize,
    cols: usize,
}

impl<const M: usize, const N: usize> ByteMatrix<M, N> {
    pub(crate) const fn new() -> Self {
        let inner = [[0; N]; M];
        Self {
            inner,
            rows: M,
            cols: N,
        }
    }

    pub(crate) fn transpose(self) -> ByteMatrix<N, M> {
        let mut inner = [[0; M]; N];
        for i in 0..self.rows {
            for j in 0..self.cols {
                inner[j][i] = self.inner[i][j];
            }
        }
        ByteMatrix {
            inner,
            rows: self.cols,
            cols: self.rows,
        }
    }

    pub(crate) fn mul_mod<const P: usize>(self, other: [[u8; P]; N]) -> ByteMatrix<M, P> {
        let mut result = [[0; P]; M];
        if other.len() == 0 {
            return result.into();
        }

        for (i, row) in self.into_iter().enumerate() {
            for j in 0..other[0].len() {
                let col = other.map(|row| row[j]);
                result[i][j] = row
                    .into_iter()
                    .zip(col.into_iter())
                    .map(|(r, c)| (((r as u16) * (c as u16)) % 256u16) as u8)
                    .fold(0, |acc, e| acc ^ e);
            }
        }
        result.into()
    }

    pub(crate) fn xor(self, other: Self) -> Self {
        let mut inner = [[0; N]; M];
        for (i, (a, b)) in self.into_iter().zip(other.into_iter()).enumerate() {
            inner[i] = xor(a, b)
        }
        inner.into()
    }
}

impl<const M: usize, const N: usize> From<[[u8; N]; M]> for ByteMatrix<M, N> {
    fn from(arr: [[u8; N]; M]) -> Self {
        Self {
            inner: arr,
            rows: M,
            cols: N,
        }
    }
}

#[derive(Debug)]
pub(crate) enum TryFromArrayError {
    InvalidLen {
        len: usize,
        cols: usize,
        rows: usize,
    },
}

impl std::fmt::Display for TryFromArrayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TryFromArrayError::InvalidLen { len, cols, rows } => write!(
                f,
                "Cannot convert array of len {len} into a matrix of size {rows}x{cols}"
            ),
        }
    }
}

impl<const M: usize, const N: usize> TryFrom<&[u8]> for ByteMatrix<M, N> {
    type Error = TryFromArrayError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut matrix = ByteMatrix::new();
        if value.len() != matrix.rows * matrix.cols {
            return Err(TryFromArrayError::InvalidLen {
                len: value.len(),
                rows: matrix.rows,
                cols: matrix.cols,
            });
        }

        for i in 0..matrix.rows {
            for j in 0..matrix.cols {
                matrix.inner[i][j] = value[i * matrix.cols + j]
            }
        }
        Ok(matrix)
    }
}

impl<const M: usize, const N: usize, const S: usize> TryFrom<[u8; S]> for ByteMatrix<M, N> {
    type Error = TryFromArrayError;

    fn try_from(value: [u8; S]) -> Result<Self, Self::Error> {
        let mut matrix = ByteMatrix::new();
        if value.len() != matrix.rows * matrix.cols {
            return Err(TryFromArrayError::InvalidLen {
                len: value.len(),
                rows: matrix.rows,
                cols: matrix.cols,
            });
        }

        for i in 0..matrix.rows {
            for j in 0..matrix.cols {
                matrix.inner[i][j] = value[i * matrix.cols + j]
            }
        }
        Ok(matrix)
    }
}

impl<const M: usize, const N: usize> IntoIterator for ByteMatrix<M, N> {
    type Item = [u8; N];
    type IntoIter = std::array::IntoIter<Self::Item, M>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose() {
        let input = ByteMatrix::from([[1, 2, 3], [4, 5, 6]]);
        let result = [[1, 4], [2, 5], [3, 6]];
        assert_eq!(input.transpose(), result.into());

        let input = ByteMatrix::from([[2, 3, 1, 1], [1, 2, 3, 1], [1, 1, 2, 3], [3, 1, 1, 2]]);
        let result = [[2, 1, 1, 3], [3, 2, 1, 1], [1, 3, 2, 1], [1, 1, 3, 2]];
        assert_eq!(input.transpose(), result.into());

        let input = ByteMatrix::from([[], [], [], []]);
        let result: [[u8; 4]; 0] = [];
        assert_eq!(input.transpose(), result.into());

        let input = ByteMatrix::from([]);
        let result: [[u8; 0]; 0] = [];
        assert_eq!(input.transpose(), result.into());
    }

    #[test]
    fn mul_mod() {
        let a = ByteMatrix::from([[1, 2, 3], [4, 5, 6]]);
        let b = [[7, 8], [9, 10], [11, 12]];
        let c = [[52, 56], [115, 90]];
        assert_eq!(a.mul_mod(b.into()), c.into());

        let a = ByteMatrix::from([[2, 3, 1, 1], [1, 2, 3, 1], [1, 1, 2, 3], [3, 1, 1, 2]]);
        let b = [[32], [81], [21], [106]];
        let c = [[204], [215], [101], [240]];
        assert_eq!(a.mul_mod(b.into()), c.into());

        let a = ByteMatrix::from([[2, 3, 1, 1], [1, 2, 3, 1], [1, 1, 2, 3], [3, 1, 1, 2]]);
        let b = [[], [], [], []];
        let c = [[], [], [], []];
        assert_eq!(a.mul_mod(b.into()), c.into());

        let a = ByteMatrix::from([[], [], [], []]);
        let b = [];
        let c = [[], [], [], []];
        assert_eq!(a.mul_mod(b.into()), c.into());
    }

    #[test]
    fn try_from() {
        assert_eq!(
            ByteMatrix::<2, 3>::try_from([1, 2, 3, 4, 5, 6]).unwrap(),
            ByteMatrix {
                inner: [[1, 2, 3], [4, 5, 6]],
                rows: 2,
                cols: 3
            }
        );
    }
}
