use log::debug;
use puzzled_polyform::{CoordBox, Poly, Polyform, TrimSides, UnitPoly};

/// Represents a 2D board for the puzzle, where each cell is either true (filled) or false (empty).
/// A filled cell is either outside the puzzle area or blocked by a placed tile.
/// An empty cell is not blocked by a tile and a tile can be placed there.
#[derive(Clone, Debug)]
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
    /// use puzzled_solver::board::Board;
    /// use puzzled_polyform::*;
    ///
    /// let board = Board::new(Polyomino::sized(RegularCoord::new(3, 4), ()).into());
    /// assert_eq!(board.get_polyform(), &Polyomino::sized(RegularCoord::new(3, 4), ()).into());
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
    /// use puzzled_solver::board::Board;
    /// use puzzled_polyform::*;
    ///
    /// let board = Board::new(Polyomino::sized(RegularCoord::new(3, 4), ()).into());
    /// assert_eq!(board.get_polyform(), &Polyomino::sized(RegularCoord::new(3, 4), ()).into());
    /// ```
    pub fn get_polyform(&self) -> &Polyform<()> {
        &self.0
    }

    /// Prints the board to the debug log.
    #[allow(dead_code)]
    pub(crate) fn debug_print(&self) {
        if log::log_enabled!(log::Level::Debug) {
            debug!("Board:");
        }
    }

    /// Trims the board by removing any rows or columns on the edges that are entirely
    /// true (filled).
    pub(crate) fn trim(&mut self) -> TrimSides<CoordBox> {
        self.0 = self.0.inverted();
        let trim = self.0.trim();
        self.0 = self.0.inverted();
        trim
    }
}

impl From<Polyform<()>> for Board {
    fn from(array: Polyform<()>) -> Self {
        Board(array)
    }
}
