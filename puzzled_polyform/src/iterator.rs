use crate::Polyform;
use crate::coord::hex::HexCoord;
use crate::coord::regular::RegularCoord;
use crate::poly::Poly;
use crate::prototile::{Hexagon, PrototileMutRef, PrototileRef, Square};
use std::hash::Hash;
use std::slice::IterMut;

impl<T> Polyform<T>
where
    T: Clone + Hash,
{
    pub fn iter(&self) -> PolyformIter<'_, T> {
        PolyformIter {
            index: 0,
            polyform: self,
        }
    }

    pub fn iter_mut(&mut self) -> PolyformIterMut<'_, T> {
        match self {
            Polyform::Polyomino(polyomino) => {
                PolyformIterMut::Polyomino(polyomino.cells_mut().iter_mut())
            }
            Polyform::Polyhex(polyhex) => PolyformIterMut::Polyhex(polyhex.cells_mut().iter_mut()),
        }
    }

    pub fn rotations_flips_iter(
        &self,
        allow_rotation: bool,
        allow_flip: bool,
    ) -> PolyformRotationFlipsIter<T> {
        PolyformRotationFlipsIter {
            index: 0,
            polyform: self.clone(),
            allow_rotation,
            allow_flip,
        }
    }
}

pub struct PolyformIter<'a, T>
where
    T: Clone + Hash,
{
    index: usize,
    polyform: &'a Polyform<T>,
}

impl<'a, T> Iterator for PolyformIter<'a, T>
where
    T: Clone + Hash,
{
    type Item = PrototileRef<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let v = match self.polyform {
            Polyform::Polyomino(polyomino) => polyomino.cells().get(self.index).map(|p| p.into()),
            Polyform::Polyhex(polyhex) => polyhex.cells().get(self.index).map(|p| p.into()),
        };
        self.index += 1;
        v
    }
}

pub enum PolyformIterMut<'a, T>
where
    T: Clone + Hash,
{
    Polyomino(IterMut<'a, Square<T, RegularCoord>>),
    Polyhex(IterMut<'a, Hexagon<T, HexCoord>>),
}

impl<'a, T> Iterator for PolyformIterMut<'a, T>
where
    T: Clone + Hash,
{
    type Item = PrototileMutRef<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Polyomino(iter) => iter.next().map(|s| s.into()),
            Self::Polyhex(iter) => iter.next().map(|h| h.into()),
        }
    }
}

pub struct PolyformRotationFlipsIter<T>
where
    T: Clone + Hash,
{
    index: usize,
    polyform: Polyform<T>,
    allow_rotation: bool,
    allow_flip: bool,
}

impl<T> Iterator for PolyformRotationFlipsIter<T>
where
    T: Clone + Hash,
{
    type Item = Polyform<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.polyform {
            Polyform::Polyomino(_) => {
                let polyform = &mut self.polyform;

                if self.allow_rotation && self.allow_flip {
                    let v = if self.index >= 8 {
                        None
                    } else if self.index == 4 {
                        polyform.flip(RegularCoord::new(0, 0).into());
                        Some(polyform.clone())
                    } else {
                        polyform.rotate_around(RegularCoord::new(0, 0).into());
                        Some(polyform.clone())
                    };
                    self.index += 1;
                    v
                } else if self.allow_rotation && !self.allow_flip {
                    let v = if self.index >= 4 {
                        None
                    } else {
                        polyform.rotate_around(RegularCoord::new(0, 0).into());
                        Some(polyform.clone())
                    };
                    self.index += 1;
                    v
                } else if !self.allow_rotation && self.allow_flip {
                    let v = if self.index >= 2 {
                        None
                    } else if self.index == 0 {
                        Some(polyform.clone())
                    } else {
                        polyform.flip(RegularCoord::new(0, 0).into());
                        Some(polyform.clone())
                    };
                    self.index += 1;
                    v
                } else {
                    let v = if self.index == 0 {
                        Some(self.polyform.clone())
                    } else {
                        None
                    };
                    self.index += 1;
                    v
                }
            }
            Polyform::Polyhex(_) => {
                todo!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Polyform, Polyomino, UnitPoly};

    #[test]
    fn test_polyomino_rotation_flip() {
        let polyomino: Polyform<()> = Polyomino::from_bool_slice(&[
            [true, false, false],
            [true, true, true],
            [true, false, true],
        ])
        .into();

        let rfs: Vec<Polyform<()>> = polyomino.rotations_flips_iter(true, true).collect();
        assert_eq!(rfs.len(), 8);
        assert_eq!(
            rfs[0],
            Polyomino::from_bool_slice(&[
                [false, true, true],
                [false, true, false],
                [true, true, true],
            ])
            .into()
        );
        assert_eq!(
            rfs[1],
            Polyomino::from_bool_slice(&[
                [true, false, true],
                [true, true, true],
                [false, false, true],
            ])
            .into()
        );
        assert_eq!(
            rfs[2],
            Polyomino::from_bool_slice(&[
                [true, true, true],
                [false, true, false],
                [true, true, false],
            ])
            .into()
        );
        assert_eq!(
            rfs[3],
            Polyomino::from_bool_slice(&[
                [true, false, false],
                [true, true, true],
                [true, false, true],
            ])
            .into()
        );
        assert_eq!(
            rfs[4],
            Polyomino::from_bool_slice(&[
                [true, false, true],
                [true, true, true],
                [true, false, false],
            ])
            .into()
        );
        assert_eq!(
            rfs[5],
            Polyomino::from_bool_slice(&[
                [true, true, false],
                [false, true, false],
                [true, true, true],
            ])
            .into()
        );
        assert_eq!(
            rfs[6],
            Polyomino::from_bool_slice(&[
                [false, false, true],
                [true, true, true],
                [true, false, true],
            ])
            .into()
        );
        assert_eq!(
            rfs[7],
            Polyomino::from_bool_slice(&[
                [true, true, true],
                [false, true, false],
                [false, true, true],
            ])
            .into()
        );
    }

    #[test]
    fn test_polyomino_no_rotation_flip() {
        let polyomino: Polyform<()> = Polyomino::from_bool_slice(&[
            [true, false, false],
            [true, true, true],
            [true, false, true],
        ])
        .into();

        let rfs: Vec<Polyform<()>> = polyomino.rotations_flips_iter(false, true).collect();
        assert_eq!(rfs.len(), 2);
        assert_eq!(
            rfs[0],
            Polyomino::from_bool_slice(&[
                [true, false, false],
                [true, true, true],
                [true, false, true],
            ])
            .into()
        );
        assert_eq!(
            rfs[1],
            Polyomino::from_bool_slice(&[
                [true, false, true],
                [true, true, true],
                [true, false, false],
            ])
            .into()
        );
    }

    #[test]
    fn test_polyomino_rotation_no_flip() {
        let polyomino: Polyform<()> = Polyomino::from_bool_slice(&[
            [true, false, false],
            [true, true, true],
            [true, false, true],
        ])
        .into();

        let rfs: Vec<Polyform<()>> = polyomino.rotations_flips_iter(true, false).collect();
        assert_eq!(rfs.len(), 4);
        assert_eq!(
            rfs[0],
            Polyomino::from_bool_slice(&[
                [false, true, true],
                [false, true, false],
                [true, true, true],
            ])
            .into()
        );
        assert_eq!(
            rfs[1],
            Polyomino::from_bool_slice(&[
                [true, false, true],
                [true, true, true],
                [false, false, true],
            ])
            .into()
        );
        assert_eq!(
            rfs[2],
            Polyomino::from_bool_slice(&[
                [true, true, true],
                [false, true, false],
                [true, true, false],
            ])
            .into()
        );
        assert_eq!(
            rfs[3],
            Polyomino::from_bool_slice(&[
                [true, false, false],
                [true, true, true],
                [true, false, true],
            ])
            .into()
        );
    }

    #[test]
    fn test_polyomino_no_rotation_no_flip() {
        let polyomino: Polyform<()> = Polyomino::from_bool_slice(&[
            [true, false, false],
            [true, true, true],
            [true, false, true],
        ])
        .into();

        let rfs: Vec<Polyform<()>> = polyomino.rotations_flips_iter(false, false).collect();
        assert_eq!(rfs.len(), 1);
        assert_eq!(
            rfs[0],
            Polyomino::from_bool_slice(&[
                [true, false, false],
                [true, true, true],
                [true, false, true],
            ])
            .into()
        );
    }
}
