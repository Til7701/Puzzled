/// Defines how a cell of a tile or board should be drawn.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub enum PrototileDrawingMode {
    /// Draw normally
    #[default]
    Normal,
    /// Draw with a highlight indicating that this cell overlaps with another tile
    Overlapping,
    /// Draw with a highlight indicating that this cell is out of bounds of the board
    OutOfBounds,
    /// General highlighting used for target selection on boards.
    Highlighted,
}
