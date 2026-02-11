use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct PuzzleCollection {
    pub name: String,
    pub description: Option<String>,
    pub author: String,
    pub id: String,
    pub version: Option<String>,
    #[serde(default = "default_true")]
    pub allow_board_rotation: bool,
    #[serde(default)]
    pub progression: Progression,
    /// Custom tiles to override or extend predefined tiles.
    pub custom_tiles: Option<HashMap<String, Tile>>,
    pub custom_boards: Option<HashMap<String, Board>>,
    pub puzzles: Vec<Puzzle>,
}

fn default_true() -> bool {
    true
}

#[derive(Deserialize, Default)]
#[serde(tag = "type")]
pub enum Progression {
    #[default]
    Any,
    Sequential,
}

#[derive(Deserialize)]
pub struct Puzzle {
    pub name: String,
    pub description: Option<String>,
    pub difficulty: Option<PuzzleDifficulty>,
    /// The tiles to use in this puzzle. Can reference predefined tiles, custom tiles or define
    /// them inline.
    pub tiles: Vec<Tile>,
    pub board: Board,
    /// Additional metadata for the puzzle.
    /// This is shown in the Puzzle Info dialog and may contain solution statistics or other info.
    pub additional_info: Option<HashMap<String, String>>,
}

#[derive(Deserialize)]
pub enum PuzzleDifficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum Tile {
    /// Can either be predefined in the application or defined in the `custom_tiles` section.
    Ref(String),
    Layout(TileLayout),
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum TileLayout {
    /// Can either be predefined in the application or defined in the `custom_tiles` section.
    Ref(String),
    Custom(Vec<Vec<i8>>),
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum Board {
    Ref(String),
    SimpleBoard {
        layout: Vec<Vec<u8>>,
    },
    AreaBoard {
        area_layout: Vec<Vec<i32>>,
        values: Vec<Vec<String>>,
        value_order: Vec<Vec<i32>>,
        areas: Vec<Area>,
        target_template: String,
    },
}

#[derive(Deserialize, Clone)]
pub struct Area {
    pub name: String,
    pub formatter: AreaFormatter,
    /// The produced value must be equal to one value in the values array of the board.
    pub default_factory: DefaultFactory,
}

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
pub enum AreaFormatter {
    Plain,
    /// Appends "st", "nd", "rd" or "th" to the value.
    Nth,
    PrefixSuffix {
        prefix: String,
        suffix: String,
    },
}

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
pub enum DefaultFactory {
    Fixed {
        value: String,
    },
    /// The current day number (1-31).
    CurrentDay,
    /// The current month in short format (e.g., "Jan", "Feb").
    CurrentMonthShort,
    /// The second digit of the current year when in two-digit format (e.g., "26" -> '2').
    CurrentYear2FirstDigit,
    /// The second digit of the current year when in two-digit format (e.g., "26" -> '6').
    CurrentYear2SecondDigit,
}
