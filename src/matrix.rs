#[derive(Clone)]
pub struct Matrix2D<T: Clone> {
    pub width: usize,
    pub height: usize,
    pub values: Vec<T>,
}

impl<T: Clone> Matrix2D<T> {
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            values: Vec::<T>::new(),
        }
    }

    #[must_use]
    pub fn row(&self, idx: usize) -> Vec<T> {
        self.values[idx * self.width..self.width * (idx + 1)].to_vec()
    }

    #[must_use]
    pub fn col(&self, idx: usize) -> Vec<T> {
        self.values
            .iter()
            .skip(idx)
            .step_by(self.width)
            .cloned()
            .collect()
    }

    #[must_use]
    pub fn cols(&self) -> Vec<Vec<T>> {
        (0..self.width).map(|idx| self.col(idx)).collect()
    }

    #[must_use]
    pub fn rows(&self) -> Vec<Vec<T>> {
        (0..self.height).map(|idx| self.row(idx)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rows() {
        let data = Matrix2D {
            width: 5,
            height: 2,
            values: vec![0_usize, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        };

        let rows = data.rows();
        assert_eq!(rows, vec![vec![0, 1, 2, 3, 4], vec![5, 6, 7, 8, 9]])
    }

    #[test]
    fn test_cols() {
        let data = Matrix2D {
            width: 5,
            height: 2,
            values: vec![0_usize, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        };

        let cols = data.cols();
        assert_eq!(
            cols,
            vec![vec![0, 5], vec![1, 6], vec![2, 7], vec![3, 8], vec![4, 9]]
        )
    }
}
