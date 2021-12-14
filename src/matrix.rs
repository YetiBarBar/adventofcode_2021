#[derive(Clone, Debug)]
#[allow(clippy::module_name_repetitions)]
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

    #[must_use]
    pub fn neighbour(&self, x: usize, y: usize, diag: bool) -> Vec<T> {
        let mut res = Vec::new();
        if let Some(left) = self.neighbour_left(x, y) {
            res.push(left);
        }
        if let Some(right) = self.neighbour_right(x, y) {
            res.push(right);
        }
        if let Some(value) = self.neighbour_up(x, y) {
            res.push(value);
        }
        if let Some(value) = self.neighbour_down(x, y) {
            res.push(value);
        }
        if diag {
            if let Some(value) = self.neighbour_down_left(x, y) {
                res.push(value);
            }
            if let Some(value) = self.neighbour_down_right(x, y) {
                res.push(value);
            }
            if let Some(value) = self.neighbour_up_right(x, y) {
                res.push(value);
            }
            if let Some(value) = self.neighbour_up_left(x, y) {
                res.push(value);
            }
        }
        res
    }

    #[must_use]
    fn x_y_to_idx(&self, x: usize, y: usize) -> Option<T> {
        if x.ge(&self.width) || y.ge(&self.height) {
            None
        } else {
            self.values.get(x + y * self.width).cloned()
        }
    }

    #[must_use]
    pub fn neighbour_left(&self, x: usize, y: usize) -> Option<T> {
        if x == 0 {
            None
        } else {
            self.x_y_to_idx(x - 1, y)
        }
    }

    #[must_use]
    pub fn neighbour_left_coord(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if x == 0 || x.ge(&self.width) || y.ge(&self.height) {
            None
        } else {
            Some((x - 1, y))
        }
    }

    #[must_use]
    pub fn neighbour_right(&self, x: usize, y: usize) -> Option<T> {
        if x.lt(&(self.width - 1)) && y.lt(&self.height) {
            self.x_y_to_idx(x + 1, y)
        } else {
            None
        }
    }

    #[must_use]
    pub fn neighbour_right_coord(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if x.lt(&(self.width - 1)) {
            Some((x + 1, y))
        } else {
            None
        }
    }

    #[must_use]
    pub fn neighbour_up(&self, x: usize, y: usize) -> Option<T> {
        if y == 0 {
            None
        } else {
            self.x_y_to_idx(x, y - 1)
        }
    }

    #[must_use]
    pub fn neighbour_up_coord(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if y == 0 || x.ge(&self.width) || y.ge(&self.height) {
            None
        } else {
            Some((x, y - 1))
        }
    }

    #[must_use]
    pub fn neighbour_down(&self, x: usize, y: usize) -> Option<T> {
        // x goes from 0..self.width
        if y.le(&(self.height - 1)) {
            self.x_y_to_idx(x, y + 1)
        } else {
            None
        }
    }

    #[must_use]
    pub fn neighbour_down_coord(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        // x goes from 0..self.width
        if y.le(&(self.height - 1)) {
            Some((x, y + 1))
        } else {
            None
        }
    }

    #[must_use]
    pub fn neighbour_up_left(&self, x: usize, y: usize) -> Option<T> {
        if x == 0 || y == 0 {
            None
        } else {
            self.x_y_to_idx(x - 1, y - 1)
        }
    }

    #[must_use]
    pub fn neighbour_up_right(&self, x: usize, y: usize) -> Option<T> {
        if x.ge(&(self.width - 1)) || y == 0 {
            None
        } else {
            self.x_y_to_idx(x + 1, y - 1)
        }
    }

    #[must_use]
    pub fn neighbour_left_up_coord(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if x == 0 || x.ge(&self.width) || y.ge(&self.height) || y == 0 {
            None
        } else {
            Some((x - 1, y - 1))
        }
    }

    #[must_use]
    pub fn neighbour_down_right(&self, x: usize, y: usize) -> Option<T> {
        if x.ge(&(self.width - 1)) || y.ge(&(self.height - 1)) {
            None
        } else {
            self.x_y_to_idx(x + 1, y + 1)
        }
    }

    #[must_use]
    pub fn neighbour_down_left(&self, x: usize, y: usize) -> Option<T> {
        if x == 0 || y.ge(&(self.height - 1)) {
            None
        } else {
            self.x_y_to_idx(x - 1, y + 1)
        }
    }

    #[must_use]
    pub fn get_x_y(&self, x: usize, y: usize) -> T {
        self.values[x + y * self.width].clone()
    }

    #[must_use]
    /// Compute neighboorhood coord
    ///
    /// # Panics
    ///
    /// If something bad happens in conversions.
    pub fn get_neighbours_coord(&self, x: usize, y: usize, diags: bool) -> Vec<(usize, usize)> {
        let deltas = {
            let mut deltas = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
            if diags {
                deltas.extend(vec![(-1, -1), (-1, 1), (1, -1), (1, 1)]);
            };
            deltas
        };

        deltas
            .iter()
            .filter_map(|&(delta_x, delta_y)| {
                let x_new: isize = isize::try_from(x).unwrap() + delta_x;
                let y_new: isize = isize::try_from(y).unwrap() + delta_y;
                if x_new.ge(&0)
                    && x_new.lt(&(isize::try_from(self.width).unwrap()))
                    && y_new.ge(&0)
                    && y_new.lt(&(isize::try_from(self.height)).unwrap())
                {
                    Some((
                        usize::try_from(x_new).unwrap(),
                        usize::try_from(y_new).unwrap(),
                    ))
                } else {
                    None
                }
            })
            .collect()
    }

    #[must_use]
    pub fn transpose(&self) -> Self {
        let values: Vec<T> = self.cols().iter().flat_map(|v| v.iter()).cloned().collect();
        Self {
            width: self.height,
            height: self.width,
            values,
        }
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

    #[test]
    fn test_neighbours() {
        let matrix = Matrix2D {
            height: 3,
            width: 4,
            values: vec![0_usize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        };
        assert_eq!(matrix.neighbour_left(1, 1), Some(4));
        assert_eq!(matrix.neighbour_right(1, 1), Some(6));
        assert_eq!(matrix.neighbour_down(1, 1), Some(9));
        assert_eq!(matrix.neighbour_up(1, 1), Some(1));
        assert_eq!(matrix.neighbour_left(3, 1), Some(6));
        assert_eq!(matrix.neighbour_right(3, 1), None);
        assert_eq!(matrix.neighbour_up(3, 0), None);
        assert_eq!(matrix.neighbour_down(3, 2), None);
        assert_eq!(matrix.neighbour_left(0, 2), None);
        assert_eq!(matrix.neighbour_down(3, 1), Some(11));
        assert_eq!(matrix.neighbour_up(3, 1), Some(3));
        assert_eq!(matrix.neighbour(0, 0, false), vec![1, 4]);
        assert_eq!(matrix.neighbour(0, 0, true), vec![1, 4, 5]);
        assert_eq!(matrix.neighbour(1, 1, false), vec![4, 6, 1, 9]);
    }
}
