use crate::tile::Tile;

pub struct Solution {
    pub placements: Vec<TilePlacement>,
}

pub struct TilePlacement {
    pub tile: Tile,
    pub position: (usize, usize),
}

pub enum UnsolvableReason {
    TooFewTiles,
    TooManyTiles,
    NoFit,
}
