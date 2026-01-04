use crate::puzzle::util::{rotate_90, transform};
use ndarray::Array2;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tile {
    pub id: i32,
    pub base: Array2<bool>,
    pub all_rotations: Vec<Array2<bool>>,
}

impl Tile {
    pub fn new(id: i32, base: Array2<bool>) -> Tile {
        let mut all_rotations_set: HashSet<Array2<bool>> = HashSet::new();

        all_rotations_set.insert(base.clone());

        let mut tmp = rotate_90(&base);
        all_rotations_set.insert(tmp.clone());
        tmp = rotate_90(&tmp);
        all_rotations_set.insert(tmp.clone());
        tmp = rotate_90(&tmp);
        all_rotations_set.insert(tmp.clone());

        tmp = transform(&mut base.clone());
        all_rotations_set.insert(tmp.clone());

        tmp = rotate_90(&tmp);
        all_rotations_set.insert(tmp.clone());
        tmp = rotate_90(&tmp);
        all_rotations_set.insert(tmp.clone());
        tmp = rotate_90(&tmp);
        all_rotations_set.insert(tmp.clone());

        let all_rotations = all_rotations_set.into_iter().collect();
        Tile {
            id,
            base,
            all_rotations,
        }
    }
}
