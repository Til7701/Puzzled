use crate::model::puzzle_meta::PuzzleMeta;

pub mod collection;
pub mod puzzle;
mod puzzle_meta;
pub mod stars;
pub mod store;

pub fn reset_metadata() {
    PuzzleMeta::new().reset();
}
