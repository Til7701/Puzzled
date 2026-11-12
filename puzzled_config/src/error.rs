#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadError {
    FileReadError(String),
    MissingVersion,
    MalformedVersion,
    UnsupportedVersion,
    JsonError(String),
    UnknownPredefinedTile {
        name: String,
    },
    UnknownCustomBoard {
        puzzle_name: String,
        board_name: String,
    },
    TileWidthOrHeightCannotBeZero,
    BoardWidthOrHeightCannotBeZero,
    InvalidVersion(String),
    InvalidCollectionId(String),
    InvalidColor {
        message: String,
    },
}
