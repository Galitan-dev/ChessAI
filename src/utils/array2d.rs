#[derive(Clone, Copy)]
pub struct Array2D<T, const W: usize, const H: usize> {
    data: [[Option<T>; W]; H],
    width: usize,
    height: usize,
}

#[allow(unused)]
impl<T: Copy, const W: usize, const H: usize> Array2D<T, W, H> {
    pub fn new() -> Self {
        Self {
            data: [[None; W]; H],
            width: W,
            height: H,
        }
    }

    pub fn set<X, Y>(&mut self, x: X, y: Y, value: T)
    where
        X: Into<usize>,
        Y: Into<usize>,
    {
        self.data[y.into()][x.into()] = Some(value);
    }

    pub fn fill_column<X>(&mut self, x: X, value: T)
    where
        X: Into<usize> + Copy,
    {
        for y in 0..self.height {
            self.data[y][x.into()] = Some(value)
        }
    }

    pub fn fill_row<Y>(&mut self, y: Y, value: T)
    where
        Y: Into<usize> + Copy,
    {
        for x in 0..self.width {
            self.data[y.into()][x] = Some(value)
        }
    }

    pub fn get<X, Y>(&self, x: X, y: Y) -> Option<T>
    where
        X: Into<usize>,
        Y: Into<usize>,
    {
        self.data[y.into()][x.into()]
    }

    pub fn delete<X, Y>(&mut self, x: X, y: Y)
    where
        X: Into<usize>,
        Y: Into<usize>,
    {
        self.data[y.into()][x.into()] = None;
    }

    pub fn has<X, Y>(&self, x: X, y: Y) -> bool
    where
        X: Into<usize>,
        Y: Into<usize>,
    {
        self.data[y.into()][x.into()].is_some()
    }
}

impl<T: Copy, const W: usize, const H: usize> IntoIterator for Array2D<T, W, H> {
    type Item = (usize, usize, T);

    type IntoIter = std::vec::IntoIter<(usize, usize, T)>;

    fn into_iter(self) -> Self::IntoIter {
        let mut vec: Vec<(usize, usize, T)> = Vec::new();

        for (y, row) in self.data.iter().enumerate() {
            for (x, item_option) in row.iter().enumerate() {
                if let Some(item) = item_option {
                    vec.push((x, y, *item));
                }
            }
        }

        vec.into_iter()
    }
}
