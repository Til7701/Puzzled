#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadError {
    FileReadError(String),
    MissingVersion,
    MalformedVersion,
    UnsupportedVersion,
    JsonError(String),
    UnknownPredefinedTile {
        tile_name: String,
        name: String,
    },
    UnknownCustomBoard {
        puzzle_name: String,
        board_name: String,
    },
    TileWidthOrHeightCannotBeZero {
        tile_name: String,
    },
    BoardWidthOrHeightCannotBeZero,
    InvalidVersion(String),
}
