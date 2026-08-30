use crate::polyform::Polyform;
use crate::polyform::prototile::Prototile;

impl<T> Polyform<T>
where
    T: Default + Clone,
{
    pub fn iter(&self) -> PolyformIter<T> {
        PolyformIter {
            index: 0,
            polyform: self,
        }
    }
}

pub struct PolyformIter<'a, T>
where
    T: Default + Clone,
{
    index: usize,
    polyform: &'a Polyform<T>,
}

impl<'a, T> Iterator for PolyformIter<'a, T>
where
    T: Default + Clone,
{
    type Item = Prototile<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.polyform {
            Polyform::Polyomino { cells: data, .. } => {
                data.get(self.index).map(|p| p.into())
            }
            Polyform::Hexomino { cells: data, .. } => {
                data.get(self.index).map(|p| p.into())
            }
        }
    }
}
