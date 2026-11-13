use crate::coord::hex::HexCoord;
use crate::poly::{Poly, TrimSides, UnitPoly};
use crate::prototile::{PrototileMutRef, PrototileRef};
use crate::{Coord, Hexagon, HexagonOrientation, MutPrototile, Prototile};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Polyhex<T>
where
    T: Clone + Hash,
{
    dim: HexCoord,
    cells: Vec<Hexagon<T, HexCoord>>,
}

impl<T> Polyhex<T>
where
    T: Clone + Hash,
{
    pub fn sized(_size: HexCoord, _value: T) -> Self {
        todo!()
    }

    pub fn from_vec<X, M>(vec: &[Vec<X>], mapper: &M) -> Self
    where
        M: Fn(&X, (usize, usize)) -> Option<T>,
    {
        let size = HexCoord::from_offset(
            vec.len() as i32,
            if !vec.is_empty() {
                vec[0].len() as i32
            } else {
                0
            },
        );
        let mut hexagons = Vec::with_capacity(size.area());
        for (i, row) in vec.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                if let Some(data) = mapper(value, (i, j)) {
                    hexagons.push(Hexagon::new(
                        HexCoord::from_offset(i as i32, j as i32),
                        HexagonOrientation::OnCorner,
                        data,
                    ))
                }
            }
        }

        Polyhex {
            dim: size,
            cells: hexagons,
        }
    }

    pub fn cells(&self) -> &[Hexagon<T, HexCoord>] {
        &self.cells
    }

    pub fn cells_mut(&mut self) -> &mut [Hexagon<T, HexCoord>] {
        &mut self.cells
    }

    fn get_hexagon(&self, coord: HexCoord) -> Option<&Hexagon<T, HexCoord>> {
        self.cells.iter().find(|p| p.coord() == coord)
    }
}

impl<T> Poly<T, HexCoord> for Polyhex<T>
where
    T: Clone + Hash,
{
    fn get(&self, coord: HexCoord) -> Option<PrototileRef<'_, T>> {
        self.get_hexagon(coord).map(|s| s.into())
    }

    fn get_mut(&mut self, coord: HexCoord) -> Option<PrototileMutRef<'_, T>> {
        self.cells
            .iter_mut()
            .find(|p| p.coord() == coord)
            .map(|s| s.into())
    }

    fn dim(&self) -> HexCoord {
        self.dim
    }

    fn relative_cartesian_dim(&self) -> (f64, f64) {
        (self.dim.q() as f64, self.dim.r() as f64) // FIXME
    }

    fn area(&self) -> usize {
        self.dim.area()
    }

    fn rotate_to_landscape(&mut self) {
        if self.dim.q() < self.dim.r() {
            self.transpose();
        }
    }

    fn rotate_around(&mut self, prime: HexCoord) -> HexCoord {
        let movement_request = { HexCoord::new(0, 0) };

        for cell in &mut self.cells {
            cell.set_coord(cell.coord().rotate_around(prime) - movement_request);
        }
        self.dim = self.dim.transpose();
        movement_request
    }

    fn flip(&mut self, prime: HexCoord) -> HexCoord {
        let movement_request = HexCoord::new(0, 0);

        for cell in &mut self.cells {
            cell.set_coord(cell.coord().flip_default(prime) - movement_request);
        }

        movement_request
    }

    fn transpose(&mut self) {
        self.dim = self.dim.transpose();
        self.cells.iter_mut().for_each(|s| s.transpose())
    }

    #[allow(refining_impl_trait_reachable)]
    fn mapped<R: Clone + Hash, M>(&self, mapper: &M) -> Polyhex<R>
    where
        M: Fn(&T) -> R,
    {
        let hexagons = self
            .cells
            .iter()
            .map(|h| {
                let data = h.data();
                let new_data = mapper(data);
                Hexagon::new(h.coord(), h.orientation(), new_data)
            })
            .collect();
        Polyhex {
            dim: self.dim,
            cells: hexagons,
        }
    }

    #[allow(refining_impl_trait_reachable)]
    fn mapped_indexed<R: Clone + Hash, M>(&self, mapper: &M) -> Polyhex<R>
    where
        M: Fn(&T, HexCoord) -> R,
    {
        let hexagons = self
            .cells
            .iter()
            .map(|h| {
                let data = h.data();
                let coord = h.coord();
                let new_data = mapper(data, coord);
                Hexagon::new(h.coord(), h.orientation(), new_data)
            })
            .collect();
        Polyhex {
            dim: self.dim,
            cells: hexagons,
        }
    }

    #[allow(refining_impl_trait_reachable)]
    fn filter_mapped<R: Clone + Hash, M>(&self, mapper: &M) -> Polyhex<R>
    where
        M: Fn(&T) -> Option<R>,
    {
        let hexagons = self
            .cells
            .iter()
            .filter_map(|h| {
                let data = h.data();
                let new_data = mapper(data);
                new_data.map(|new_data| Hexagon::new(h.coord(), h.orientation(), new_data))
            })
            .collect();
        Polyhex {
            dim: self.dim,
            cells: hexagons,
        }
    }

    fn extend_adjacent(&mut self, _value: T) {
        todo!()
    }

    fn trim(&mut self) -> TrimSides<HexCoord> {
        todo!()
    }
}

impl UnitPoly for Polyhex<()> {
    fn place_on_all_positions(&self, _child: &Self) -> Vec<Self> {
        todo!()
    }

    fn remove_parent(&mut self, parent: &Self) {
        self.cells.retain(|c| !parent.cells.contains(c));
    }

    fn inverted(&self) -> Self {
        todo!()
    }

    fn as_2d_slice(&self) -> String {
        todo!()
    }

    fn count_biggest_connected_area_of_cells(&self) -> usize {
        todo!()
    }

    fn count_smallest_connected_area_of_cells(&self) -> usize {
        todo!()
    }

    fn from_bool_slice<const N: usize>(slice: &[[bool; N]]) -> Self {
        let vec: Vec<Vec<bool>> = slice.iter().map(|&v| v.to_vec()).collect();
        Polyhex::from_vec(&vec, &|b, _| if *b { Some(()) } else { None })
    }
}
