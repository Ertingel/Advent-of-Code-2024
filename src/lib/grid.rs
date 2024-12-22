// cargo test --lib grid
// cargo test --lib grid -- --nocapture

use std::{fmt, usize};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn new(width: usize, height: usize, value: T) -> Self {
        Grid {
            data: vec![value; width * height],
            width,
        }
    }
}

impl<T> Grid<T> {
    pub fn map_str(str: &str, mapping: impl Fn(usize, usize, char) -> T) -> Self {
        let mut width: usize = 0;
        let data = str
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                width = line.len();

                line.chars()
                    .enumerate()
                    .map(|(x, char)| mapping(x, y, char))
                    .collect::<Vec<T>>()
            })
            .collect();

        Self { data, width }
    }

    pub fn get<X, Y>(&self, x: X, y: Y) -> Option<&T>
    where
        X: TryInto<usize>,
        Y: TryInto<usize>,
    {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;

        self.data.get(x + y * self.width)
    }

    pub fn set<X, Y>(&mut self, x: X, y: Y, value: T) -> Option<()>
    where
        X: TryInto<usize>,
        Y: TryInto<usize>,
    {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;

        *self.data.get_mut(x + y * self.width)? = value;

        Some(())
    }

    pub fn iter_xy(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.data
            .iter()
            .enumerate()
            .map(|(index, data)| (index % self.width(), index / self.width, data))
    }

    pub fn map<I>(&self, fun: impl Fn(usize, usize, &T) -> I) -> Grid<I> {
        Grid {
            width: self.width,
            data: self
                .data
                .iter()
                .enumerate()
                .map(|(index, data)| fun(index % self.width(), index / self.width, data))
                .collect(),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.data.len() / self.width
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T> fmt::Display for Grid<T>
where
    T: fmt::Display,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for (x, _, e) in self.iter_xy() {
            e.fmt(fmt)?;

            if x == self.width() {
                fmt.write_str("\n")?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_test() {
        let grid = Grid::map_str("123\n456\n789\nabc", |_, _, char| char);

        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 4);
    }

    #[test]
    fn indexing_test() {
        let grid = Grid::map_str("123\n456\n789\nabc", |_, _, char| char);

        assert_eq!(grid.get(0, 0), Some(&'1'));
        assert_eq!(grid.get(1, 0), Some(&'2'));
        assert_eq!(grid.get(2, 0), Some(&'3'));

        assert_eq!(grid.get(0, 1), Some(&'4'));
        assert_eq!(grid.get(1, 1), Some(&'5'));
        assert_eq!(grid.get(2, 1), Some(&'6'));

        assert_eq!(grid.get(0, 2), Some(&'7'));
        assert_eq!(grid.get(1, 2), Some(&'8'));
        assert_eq!(grid.get(2, 2), Some(&'9'));

        assert_eq!(grid.get(0, 3), Some(&'a'));
        assert_eq!(grid.get(1, 3), Some(&'b'));
        assert_eq!(grid.get(2, 3), Some(&'c'));
    }
}
