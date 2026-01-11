use crate::array_util;
use crate::bitmask::Bitmask;
use crate::board::Board;
use crate::tile::Tile;
use log::debug;
use ndarray::Array2;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

#[derive(Clone)]
pub struct PositionedTile {
    pub bitmasks: Vec<Bitmask>,
}

impl PositionedTile {
    pub fn new(tile: &Tile, board: &Board) -> Self {
        let all_placements: Vec<Array2<bool>> = tile
            .all_rotations
            .iter()
            .flat_map(|rotation| array_util::place_on_all_positions(board.get_array(), rotation))
            .map(|array| {
                let mut array = array.clone();
                array_util::remove_parent(board.get_array(), &mut array);
                array
            })
            .collect();

        let bitmasks: Vec<Bitmask> = all_placements
            .iter()
            .map(|array| Bitmask::from(array))
            .collect();

        PositionedTile { bitmasks }
    }

    pub fn print_debug(&self, board_width: i32) {
        for bitmask in self.bitmasks.iter() {
            debug!("{}", &bitmask.to_string(board_width));
        }
    }
}

pub async fn solve_filling(
    board_width: i32,
    board_bitmask: &Bitmask,
    positioned_tiles: &[PositionedTile],
    cancel_token: CancellationToken,
) -> Option<Vec<usize>> {
    if board_bitmask.all_relevant_bits_set() {
        return Some(Vec::new());
    }

    let solvers: Vec<AllFillingSolver> =
        prepare_solvers(board_width, board_bitmask, positioned_tiles, &cancel_token);
    let mut set: JoinSet<bool> = JoinSet::new();

    let result: Option<Vec<usize>> = {
        for mut solver in solvers.into_iter() {
            set.spawn(async move { solver.solve().await });
        }
        tokio::select! {
            _ = cancel_token.cancelled() => {
                debug!("Cancellation requested, aborting all solver tasks.");
                None
            }
            res = await_completion(&mut set) => {
                debug!("Solver Finished, aborting remaining solver tasks.");
                res
            }
        }
    };
    set.abort_all();
    result
}

async fn await_completion(set: &mut JoinSet<bool>) -> Option<Vec<usize>> {
    let mut result: Option<Vec<usize>> = None;
    while let Some(res) = set.join_next().await {
        match res {
            Ok(solved) => {
                if solved {
                    result = Some(Vec::new());
                    break;
                }
            }
            Err(_) => {}
        }
    }
    result
}

fn prepare_solvers(
    board_width: i32,
    board_bitmask: &Bitmask,
    positioned_tiles: &[PositionedTile],
    cancel_token: &CancellationToken,
) -> Vec<AllFillingSolver> {
    if positioned_tiles.is_empty() {
        return Vec::new();
    }
    let first_tile = positioned_tiles.first().unwrap();
    let mut solvers = Vec::with_capacity(first_tile.bitmasks.len());

    for i in 0..first_tile.bitmasks.len() {
        let placement = &first_tile.bitmasks[i];
        if board_bitmask.and_is_zero(&placement) {
            let mut board_with_placements = board_bitmask.clone();
            board_with_placements.xor(board_bitmask, placement);
            let mut used_tile_indices: Vec<usize> = vec![0; 1];
            used_tile_indices[0] = i;

            let solver = AllFillingSolver::new(
                board_width,
                &board_with_placements,
                &used_tile_indices,
                positioned_tiles,
                cancel_token.clone(),
            );

            solvers.push(solver);
        }
    }

    solvers
}

struct AllFillingSolver {
    board_width: i32,
    start_tile_index: usize,
    positioned_tiles: Vec<PositionedTile>,
    board_bitmasks: Vec<Bitmask>,
    used_tile_indices: Vec<usize>,
    tmp_bitmask: Bitmask,
    yield_counter: u32,
    cancel_token: CancellationToken,
}

impl AllFillingSolver {
    pub fn new(
        board_width: i32,
        board_bitmasks: &Bitmask,
        used_tile_indices: &[usize],
        positioned_tiles: &[PositionedTile],
        cancel_token: CancellationToken,
    ) -> Self {
        let positioned_tiles: Vec<PositionedTile> = positioned_tiles.to_vec();
        let num_tiles = positioned_tiles.len();

        let mut use_tile_indices_vec: Vec<usize> = Vec::with_capacity(num_tiles);
        for used_tile_index in used_tile_indices {
            use_tile_indices_vec.push(*used_tile_index);
        }
        for _ in used_tile_indices.len()..num_tiles {
            use_tile_indices_vec.push(0);
        }
        AllFillingSolver {
            board_width,
            start_tile_index: used_tile_indices.len(),
            positioned_tiles,
            board_bitmasks: vec![board_bitmasks.clone(); num_tiles],
            used_tile_indices: use_tile_indices_vec,
            tmp_bitmask: Bitmask::new(board_bitmasks.get_relevant_bits()),
            yield_counter: 0,
            cancel_token,
        }
    }

    pub async fn solve(&mut self) -> bool {
        self.solve_recursive(self.start_tile_index).await
    }

    async fn solve_recursive(&mut self, tile_index: usize) -> bool {
        self.yield_counter += 1;
        if self.yield_counter & 0xf == 0 {
            tokio::task::yield_now().await;
            if self.cancel_token.is_cancelled() {
                return false;
            }
        }

        if tile_index >= self.positioned_tiles.len() {
            return self.submit_solution();
        }

        let num_placements = self.positioned_tiles[tile_index].bitmasks.len();
        for i in 0..num_placements {
            let placement = &self.positioned_tiles[tile_index].bitmasks[i];
            if self.board_bitmasks[tile_index - 1].and_is_zero(&placement) {
                self.tmp_bitmask
                    .xor(&self.board_bitmasks[tile_index - 1], &placement);
                self.used_tile_indices[tile_index] = i;
                self.board_bitmasks[tile_index] = self.tmp_bitmask.clone();
                if Box::pin(async { self.solve_recursive(tile_index + 1).await }).await {
                    return true;
                }
            }
        }

        false
    }

    fn submit_solution(&self) -> bool {
        debug!("Submitting solution...");
        let board_filled = self.board_bitmasks.last().unwrap().all_relevant_bits_set();
        if board_filled {
            debug!(
                "Solution found with tile placements: {:?}",
                self.used_tile_indices
            );
        }
        board_filled
    }

    pub fn print_debug(&self) {
        debug!("RecursiveSolver Debug Info:");
        debug!("Board Width: {}", self.board_width);
        debug!("Start Tile Index: {}", self.start_tile_index);
        debug!("Used Tile Indices: {:?}", self.used_tile_indices);
        for (i, bitmask) in self.board_bitmasks.iter().enumerate() {
            debug!(
                "Board Bitmask after tile {}: {}",
                i,
                bitmask.to_string(self.board_width)
            );
        }
    }
}
