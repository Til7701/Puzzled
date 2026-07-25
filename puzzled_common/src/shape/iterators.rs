use crate::shape::Polyform;

impl Polyform {
    /// Iterates over the values of the shape.
    pub fn iter(&self) -> PolyformIter {
        PolyformIter {
            polyform: self,
            index: (0, 0),
        }
    }

    /// Iterates over the values of the shape while also being given the index of the value.
    pub fn indexed_iter(&self) -> PolyformIndexedIter {
        PolyformIndexedIter {
            polyform: self,
            index: (0, 0),
        }
    }

    /// Iterates over all rotations of the shape.
    /// Duplicates are not removed.
    pub fn rotations_flips_iter(&self) -> ShapeRotationIterator {
        ShapeRotationIterator::new(self.clone())
    }
}

pub struct PolyformIter<'a> {
    polyform: &'a Polyform,
    index: (usize, usize),
}

impl<'a> Iterator for PolyformIter<'a> {
    type Item = &'a bool;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.polyform.get(self.index);
        let dim = self.polyform.dim();
        if self.index.1 >= dim.1 {
            self.index.1 = 0;
            self.index.0 += 1;
        } else {
            self.index.1 += 1;
        }

        value
    }
}

pub struct PolyformIndexedIter<'a> {
    polyform: &'a Polyform,
    index: (usize, usize),
}

impl<'a> Iterator for PolyformIndexedIter<'a> {
    type Item = ((usize, usize), &'a bool);

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.polyform.get(self.index);
        let index = self.index;
        let dim = self.polyform.dim();
        if self.index.1 >= dim.1 {
            self.index.1 = 0;
            self.index.0 += 1;
        } else {
            self.index.1 += 1;
        }

        value.map(|v| (index, v))
    }
}

pub struct ShapeRotationIterator {
    current: Polyform,
    iteration: u8,
}

impl ShapeRotationIterator {
    fn new(shape: Polyform) -> Self {
        Self {
            current: shape,
            iteration: 0,
        }
    }
}

impl Iterator for ShapeRotationIterator {
    type Item = Polyform;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iteration >= 8 {
            return None;
        }
        if self.iteration == 4 {
            self.current.transpose();
        }
        let current = self.current.clone();
        self.current.rotate_counterclockwise();
        self.iteration += 1;
        Some(current)
    }
}

#[cfg(test)]
mod test {
    use crate::shape::shape_square;

    #[test]
    fn test_tile_rotation_iterator() {
        let base = shape_square(&[[true, false], [false, false]]);
        let mut iter = base.rotations_flips_iter();

        assert_eq!(
            iter.next(),
            Some(shape_square(&[[true, false], [false, false]]))
        );
        assert_eq!(
            iter.next(),
            Some(shape_square(&[[false, false], [true, false]]))
        );
        assert_eq!(
            iter.next(),
            Some(shape_square(&[[false, false], [false, true]]))
        );
        assert_eq!(
            iter.next(),
            Some(shape_square(&[[false, true], [false, false]]))
        );

        assert_eq!(
            iter.next(),
            Some(shape_square(&[[true, false], [false, false]]))
        );
        assert_eq!(
            iter.next(),
            Some(shape_square(&[[false, false], [true, false]]))
        );
        assert_eq!(
            iter.next(),
            Some(shape_square(&[[false, false], [false, true]]))
        );
        assert_eq!(
            iter.next(),
            Some(shape_square(&[[false, true], [false, false]]))
        );
        assert_eq!(iter.next(), None);
    }
}
