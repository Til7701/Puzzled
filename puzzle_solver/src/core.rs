use crate::array_util;
use crate::bitmask::Bitmask;
use crate::board::Board;
use crate::tile::Tile;
use log::debug;

#[derive(Clone)]
pub struct PositionedTile {
    pub tile_index: usize,
    pub bitmasks: Vec<Bitmask>,
}

impl PositionedTile {
    pub fn new(tile_index: usize, tile: &Tile, board: &Board) -> Self {
        let all_placements = array_util::place_on_all_positions(&tile.base, board.get_array());
        let bitmasks: Vec<Bitmask> = all_placements
            .iter()
            .map(|array| Bitmask::from(array))
            .collect();

        PositionedTile {
            tile_index,
            bitmasks,
        }
    }
}

pub fn solve_filling(
    board_bitmask: &Bitmask,
    positioned_tiles: &[PositionedTile],
) -> Option<Vec<usize>> {
    let mut solvers = prepare_solvers(board_bitmask, positioned_tiles);

    for solver in solvers.iter_mut().rev() {
        if solver.solve() {
            debug!("Solved with placements: {:?}", solver.used_tile_indices);
            return Some(solver.used_tile_indices.clone());
        }
    }

    debug!("No solution found");
    None
}

fn prepare_solvers(
    board_bitmask: &Bitmask,
    positioned_tiles: &[PositionedTile],
) -> Vec<RecursiveSolver> {
    let first_tile = positioned_tiles.first().unwrap();
    let mut solvers = Vec::with_capacity(first_tile.bitmasks.len());

    for i in 0..first_tile.bitmasks.len() {
        let placement = &first_tile.bitmasks[i];
        if board_bitmask.and_is_zero(&placement) {
            let mut tmp_bitmask = Bitmask::new();
            tmp_bitmask.xor(board_bitmask, placement);

            let mut used_tile_indices: Vec<usize> = vec![0; positioned_tiles.len()];
            used_tile_indices[0] = i;

            let solver = RecursiveSolver {
                start_tile_index: 1,
                positioned_tiles: positioned_tiles.to_vec(),
                board_bitmasks: vec![tmp_bitmask.clone(); positioned_tiles.len()],
                used_tile_indices,
                tmp_bitmask: Bitmask::new(),
            };

            solvers.push(solver);
        }
    }

    solvers
}

struct RecursiveSolver {
    start_tile_index: usize,
    pub positioned_tiles: Vec<PositionedTile>,
    pub board_bitmasks: Vec<Bitmask>,
    pub used_tile_indices: Vec<usize>,
    pub tmp_bitmask: Bitmask,
}

impl RecursiveSolver {
    pub fn new(
        board_bitmask: &Bitmask,
        used_tile_indices: &[usize],
        positioned_tiles: &[PositionedTile],
    ) -> Self {
        let positioned_tiles: Vec<PositionedTile> = positioned_tiles.to_vec();
        let num_tiles = positioned_tiles.len();

        let mut use_tile_indices_vec: Vec<usize> = Vec::with_capacity(num_tiles);
        for used_tile_index in used_tile_indices {
            use_tile_indices_vec.push(*used_tile_index);
        }
        RecursiveSolver {
            start_tile_index: used_tile_indices.len(),
            positioned_tiles,
            board_bitmasks: vec![board_bitmask.clone(); num_tiles],
            used_tile_indices: use_tile_indices_vec,
            tmp_bitmask: Bitmask::new(),
        }
    }

    pub fn solve(&mut self) -> bool {
        self.solve_recursive(self.start_tile_index)
    }

    pub fn solve_recursive(&mut self, tile_index: usize) -> bool {
        if tile_index >= self.positioned_tiles.len() {
            return true;
        }

        let num_placements = self.positioned_tiles[tile_index].bitmasks.len();
        for i in 0..num_placements {
            let placement = &self.positioned_tiles[tile_index].bitmasks[i];

            if self.board_bitmasks[tile_index - 1].and_is_zero(&placement) {
                self.tmp_bitmask
                    .xor(&self.board_bitmasks[tile_index - 1], &placement);
                self.used_tile_indices[tile_index] = i;
                self.board_bitmasks[tile_index] = self.tmp_bitmask.clone();
                if self.solve_recursive(tile_index + 1) {
                    return true;
                }
            }
        }

        false
    }
}
