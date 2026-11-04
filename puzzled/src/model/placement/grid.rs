use puzzled_common::polyform::coord::Coord;

/// Configuration for the puzzle grid layout.
#[derive(Debug, Eq, PartialEq)]
pub struct GridConfig<C: Coord> {
    pub grid_cells: C,
    pub min_grid_cells: C,
    pub cell_size_pixel: u32,
}

impl<C: Coord + Default> Default for GridConfig<C> {
    fn default() -> Self {
        GridConfig {
            grid_cells: C::default(),
            min_grid_cells: C::default(),
            cell_size_pixel: 1,
        }
    }
}
