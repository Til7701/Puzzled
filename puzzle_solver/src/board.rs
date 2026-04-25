use log::debug;
use puzzled_common::shape::TrimSides;
use puzzled_common::Shape;
use puzzled_common::ShapeType::Square;
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
pub struct Board(Shape);

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
    /// assert_eq!(board.get_shape().dim(), (3, 4));
    /// assert!(board.get_shape().iter().all(|&b| b == false));
    /// ```
    pub fn new(dims: (usize, usize)) -> Self {
        Board(Shape::from_elem(dims, Square, false))
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
    /// use puzzled_common::Shape;
    /// use puzzled_common::ShapeType::Square;
    ///
    /// let board = Board::new((3, 4));
    /// assert_eq!(board.get_shape(), &Shape::from_elem((3, 4), Square, false));
    /// ```
    pub fn get_shape(&self) -> &Shape {
        &self.0
    }

    /// Prints the board to the debug log.
    #[allow(dead_code)]
    pub(crate) fn debug_print(&self) {
        if log::log_enabled!(log::Level::Debug) {
            debug!("Board:");
            self.0.debug_print();
        }
    }

    /// Trims the board by removing any rows or columns on the edges that are entirely
    /// true (filled).
    pub(crate) fn trim(&mut self) -> TrimSides {
        self.0.trim_matching(true)
    }
}

impl Index<[usize; 2]> for Board {
    type Output = bool;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.0[(index[0], index[1])]
    }
}

impl IndexMut<[usize; 2]> for Board {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.0[(index[0], index[1])]
    }
}

impl From<Shape> for Board {
    fn from(array: Shape) -> Self {
        Board(array)
    }
}

#[cfg(test)]
mod tests {
    use super::Board;

    #[test]
    fn test_new_0_0() {
        let board = Board::new((0, 0));
        assert_eq!(board.get_shape().dim(), (0, 0));
    }

    #[test]
    fn test_new_3_4() {
        let board = Board::new((3, 4));
        assert_eq!(board.get_shape().dim(), (3, 4));
        assert!(board.get_shape().iter().all(|&b| b == false));
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

        assert_eq!(board.get_shape().dim(), (3, 3));
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
