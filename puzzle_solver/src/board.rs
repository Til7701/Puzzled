use log::debug;
use puzzled_common::polyform::{Polyform, TrimSides};

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
#[derive(Clone)]
pub struct Board(Polyform<()>);

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
    pub fn new(base: Polyform<()>) -> Self {
        Board(base)
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
    pub fn get_polyform(&self) -> &Polyform<()> {
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
        self.0.trim()
    }
}

impl From<Polyform<()>> for Board {
    fn from(array: Polyform<()>) -> Self {
        Board(array)
    }
}
