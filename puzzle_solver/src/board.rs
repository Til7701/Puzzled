use crate::array_util;
use crate::array_util::TrimSides;
use log::debug;
use ndarray::Array2;
use std::ops::{Index, IndexMut};

/// Represents a 2D board for the puzzle, where each cell is either true (filled) or false (empty).
/// A filled cell is either outside the puzzle area or blocked by a placed tile.
/// An empty cell is not blocked by a tile and a tile can be placed there.
///
/// # Examples
///
/// This creates a 5x5 board and sets the cell at (2, 3) to true (filled).
///
/// ```rust
/// use puzzle_solver::board::Board;
///
/// let mut board = Board::new((5, 5));
/// board[[2, 3]] = true;
/// assert_eq!(board[[2, 3]], true);
/// ```
pub struct Board(Array2<bool>);

impl Board {
    /// Creates a new Board with the given dimensions, initialized to all false (empty).
    ///
    /// # Arguments
    ///
    /// * `dims`: A tuple representing the dimensions of the board (x, y).
    ///
    /// returns: Board
    ///
    /// # Examples
    ///
    /// ```rust
    /// use puzzle_solver::board::Board;
    ///
    /// let board = Board::new((3, 4));
    /// assert_eq!(board.get_array().shape(), &[3, 4]);
    /// assert!(board.get_array().iter().all(|&b| b == false));
    /// ```
    pub fn new(dims: (usize, usize)) -> Self {
        Board(Array2::default(dims))
    }

    /// Returns a reference to the internal 2D array representing the board.
    ///
    /// Mutable access to the board should be done via indexing.
    ///
    /// # Arguments
    ///
    /// returns: Board
    ///
    /// # Examples
    ///
    /// ```rust
    /// use puzzle_solver::board::Board;
    /// use ndarray::Array2;
    ///
    /// let board = Board::new((3, 4));
    /// assert_eq!(board.get_array(), Array2::default((3, 4)));
    /// ```
    pub fn get_array(&self) -> &Array2<bool> {
        &self.0
    }

    /// Prints the board to the debug log.
    #[allow(dead_code)]
    pub(crate) fn debug_print(&self) {
        if log::log_enabled!(log::Level::Debug) {
            debug!("Board:");
            array_util::debug_print(&self.0);
        }
    }

    /// Trims the board by removing any rows or columns on the edges that are entirely
    /// true (filled).
    pub(crate) fn trim(&mut self) -> TrimSides {
        array_util::remove_true_rows_cols_from_sides(&mut self.0)
    }
}

impl Index<[usize; 2]> for Board {
    type Output = bool;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.0[[index[0], index[1]]]
    }
}

impl IndexMut<[usize; 2]> for Board {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.0[[index[0], index[1]]]
    }
}

impl From<Array2<bool>> for Board {
    fn from(array: Array2<bool>) -> Self {
        Board(array)
    }
}

#[cfg(test)]
mod tests {
    use super::Board;

    #[test]
    fn test_new_0_0() {
        let board = Board::new((0, 0));
        assert_eq!(board.get_array().shape(), &[0, 0]);
    }

    #[test]
    fn test_new_3_4() {
        let board = Board::new((3, 4));
        assert_eq!(board.get_array().shape(), &[3, 4]);
        assert!(board.get_array().iter().all(|&b| b == false));
    }

    #[test]
    fn test_trim() {
        let mut board = Board::new((5, 5));
        // Set edges
        for i in 0..5 {
            board[[0, i]] = true;
            board[[4, i]] = true;
            board[[i, 0]] = true;
            board[[i, 4]] = true;
        }
        // cross shape in the center
        board[[2, 1]] = false;
        board[[2, 2]] = false;
        board[[2, 3]] = false;
        board[[1, 2]] = false;
        board[[3, 2]] = false;

        board.trim();

        assert_eq!(board.get_array().shape(), &[3, 3]);
        assert_eq!(board[[0, 0]], false);
        assert_eq!(board[[0, 1]], false);
        assert_eq!(board[[0, 2]], false);
        assert_eq!(board[[1, 0]], false);
        assert_eq!(board[[1, 1]], false);
        assert_eq!(board[[1, 2]], false);
        assert_eq!(board[[2, 0]], false);
        assert_eq!(board[[2, 1]], false);
        assert_eq!(board[[2, 2]], false);
    }
}
