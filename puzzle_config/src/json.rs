use crate::config::tile;
use crate::{
    AreaConfig, AreaValueFormatter, BoardConfig, PuzzleConfig, PuzzleConfigCollection, ReadError,
    TargetTemplate, TileConfig,
};
use ndarray::Array2;
use serde::Deserialize;
use std::collections::HashMap;
use time::OffsetDateTime;

#[derive(Deserialize)]
struct PuzzleCollection {
    name: String,
    description: Option<String>,
    author: String,
    /// Custom tiles to override or extend predefined tiles.
    custom_tiles: Option<HashMap<String, Tile>>,
    custom_boards: Option<HashMap<String, Board>>,
    puzzles: Vec<Puzzle>,
}

#[derive(Deserialize)]
struct Puzzle {
    name: String,
    description: Option<String>,
    /// The tiles to use in this puzzle. Can reference predefined tiles, custom tiles or define
    /// them inline.
    tiles: Vec<Tile>,
    board: Board,
    /// Additional metadata for the puzzle.
    /// This is shown in the Puzzle Info dialog and may contain solution statistics or other info.
    additional_info: Option<HashMap<String, String>>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Tile {
    /// Can either be predefined in the application or defined in the `custom_tiles` section.
    Ref(String),
    Custom(Vec<Vec<i8>>),
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Board {
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

#[derive(Deserialize)]
struct Area {
    name: String,
    formatter: AreaFormatter,
    /// The produced value must be equal to one value in the values array of the board.
    default_factory: DefaultFactory,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum AreaFormatter {
    Plain,
    /// Appends "st", "nd", "rd" or "th" to the value.
    Nth,
    PrefixSuffix {
        prefix: String,
        suffix: String,
    },
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum DefaultFactory {
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

pub fn load_puzzle_collection_from_json(
    json_data: &str,
) -> Result<PuzzleConfigCollection, ReadError> {
    let result = serde_json::from_str::<PuzzleCollection>(json_data);
    match result {
        Ok(collection) => convert(collection),
        Err(e) => Err(ReadError::JsonError(e.to_string())),
    }
}

fn convert(puzzle_collection: PuzzleCollection) -> Result<PuzzleConfigCollection, ReadError> {
    let mut custom_tiles = HashMap::new();
    if let Some(tiles) = puzzle_collection.custom_tiles {
        for (name, tile) in tiles {
            let converted_tile = convert_tile(&name, tile, &custom_tiles)?;
            custom_tiles.insert(name.clone(), converted_tile);
        }
    }

    let mut custom_boards = HashMap::new();
    if let Some(boards) = puzzle_collection.custom_boards {
        for (name, board) in boards {
            let converted_board = convert_board(board, &custom_boards)?;
            custom_boards.insert(name.clone(), converted_board);
        }
    }

    let mut puzzle_configs = Vec::new();
    for puzzle in puzzle_collection.puzzles {
        let mut tiles = Vec::new();
        for (i, tile) in puzzle.tiles.into_iter().enumerate() {
            let tile_name = format!("puzzle '{}' tile #{}", puzzle.name, i + 1);
            let converted_tile = convert_tile(&tile_name, tile, &custom_tiles)?;
            tiles.push(converted_tile);
        }

        let board_config = convert_board(puzzle.board, &custom_boards)?;
        let puzzle_config = PuzzleConfig::new(
            puzzle.name,
            puzzle.description,
            tiles,
            board_config,
            puzzle.additional_info,
        );
        puzzle_configs.push(puzzle_config);
    }

    Ok(PuzzleConfigCollection::new(
        puzzle_collection.name,
        puzzle_collection.description,
        puzzle_collection.author,
        puzzle_configs,
    ))
}

fn convert_tile(
    tile_name: &String,
    tile: Tile,
    custom_tiles: &HashMap<String, TileConfig>,
) -> Result<TileConfig, ReadError> {
    match tile {
        Tile::Ref(name) => {
            if let Some(predefined_tile) = tile::from_predefined_tile(&name) {
                Ok(predefined_tile)
            } else if let Some(custom_tile) = custom_tiles.get(&name) {
                Ok(custom_tile.clone())
            } else {
                Err(ReadError::UnknownPredefinedTile {
                    tile_name: tile_name.clone(),
                    name,
                })
            }
        }
        Tile::Custom(array) => {
            let height = array.len();
            if height == 0 {
                return Err(ReadError::TileWidthOrHeightCannotBeZero {
                    tile_name: tile_name.clone(),
                });
            }
            let width = array[0].len();
            for row in &array {
                if row.len() != width {
                    return Err(ReadError::TileWidthOrHeightCannotBeZero {
                        tile_name: tile_name.clone(),
                    });
                }
            }
            let mut base = Array2::<bool>::default((height, width));
            for (i, row) in array.iter().enumerate() {
                for (j, &value) in row.iter().enumerate() {
                    base[(i, j)] = value != 0;
                }
            }
            Ok(TileConfig::new(base))
        }
    }
}

fn convert_board(
    board: Board,
    custom_boards: &HashMap<String, BoardConfig>,
) -> Result<BoardConfig, ReadError> {
    match { board } {
        Board::Ref(name) => {
            if let Some(custom_board) = custom_boards.get(&name) {
                Ok(custom_board.clone())
            } else {
                Err(ReadError::UnknownCustomBoard { name })
            }
        }
        Board::SimpleBoard { layout } => {
            let height = layout.len();
            if height == 0 {
                return Err(ReadError::BoardWidthOrHeightCannotBeZero);
            }
            let width = layout[0].len();
            for row in &layout {
                if row.len() != width {
                    return Err(ReadError::BoardWidthOrHeightCannotBeZero);
                }
            }
            let mut array = Array2::<bool>::default((height, width));
            for (i, row) in layout.iter().enumerate() {
                for (j, &value) in row.iter().enumerate() {
                    array[(i, j)] = value >= 0;
                }
            }
            Ok(BoardConfig::Simple { layout: array })
        }
        Board::AreaBoard {
            area_layout,
            values,
            value_order,
            areas,
            target_template,
        } => {
            let area_configs = areas
                .iter()
                .map(|a| convert_area(a))
                .collect::<Result<Vec<AreaConfig>, ReadError>>()?;

            let board_layout = {
                let height = area_layout.len();
                if height == 0 {
                    return Err(ReadError::BoardWidthOrHeightCannotBeZero);
                }
                let width = area_layout[0].len();
                for row in &area_layout {
                    if row.len() != width {
                        return Err(ReadError::BoardWidthOrHeightCannotBeZero);
                    }
                }
                let mut array = Array2::<bool>::default((height, width));
                for (i, row) in area_layout.iter().enumerate() {
                    for (j, &value) in row.iter().enumerate() {
                        array[(i, j)] = value >= 0;
                    }
                }
                array
            };

            Ok(BoardConfig::Area {
                layout: board_layout,
                area_indices: vec_vec_to_array2(&area_layout),
                display_values: vec_vec_to_array2(&values),
                value_order: vec_vec_to_array2(&value_order),
                area_configs,
                target_template: TargetTemplate::new(&target_template),
            })
        }
    }
}

fn vec_vec_to_array2<T: Clone + Default>(data: &Vec<Vec<T>>) -> Array2<T> {
    let height = data.len();
    let width = if height > 0 { data[0].len() } else { 0 };
    let mut array = Array2::<T>::default((height, width));
    for (i, row) in data.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            array[(i, j)] = value.clone();
        }
    }
    array
}

fn convert_area(area: &Area) -> Result<AreaConfig, ReadError> {
    let formatter = match &area.formatter {
        AreaFormatter::Plain => AreaValueFormatter::Plain,
        AreaFormatter::Nth => AreaValueFormatter::Nth,
        AreaFormatter::PrefixSuffix { prefix, suffix } => AreaValueFormatter::PrefixSuffix {
            prefix: prefix.clone(),
            suffix: suffix.clone(),
        },
    };

    Ok(AreaConfig::new(
        area.name.clone(),
        formatter,
        convert_default_factory(&area.default_factory)?,
    ))
}

fn convert_default_factory(factory: &DefaultFactory) -> Result<String, ReadError> {
    match factory {
        DefaultFactory::Fixed { value } => Ok(value.to_string()),
        DefaultFactory::CurrentDay => {
            let date = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
            Ok(date.day().to_string())
        }
        DefaultFactory::CurrentMonthShort => {
            let date = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
            let month_str = &date.month().to_string()[0..3];
            Ok(month_str.to_string())
        }
        DefaultFactory::CurrentYear2FirstDigit => {
            let date = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
            let year = date.year() % 100;
            let first_digit = year / 10;
            Ok(first_digit.to_string())
        }
        DefaultFactory::CurrentYear2SecondDigit => {
            let date = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
            let year = date.year() % 100;
            let second_digit = year % 10;
            Ok(second_digit.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    #[test]
    fn test_convert_predefined_tile() {
        let tile = Tile::Ref("L3".to_string());
        let converted_tile = convert_tile(&"test".to_string(), tile, &HashMap::new()).unwrap();
        let expected_tile = TileConfig::new(arr2(&[[true, false], [true, true]]));
        assert_eq!(converted_tile.base(), expected_tile.base());
    }

    #[test]
    fn test_convert_predefined_tile_unknown() {
        let tile = Tile::Ref("test".to_string());
        let converted_tile = convert_tile(&"test".to_string(), tile, &HashMap::new());
        assert!(converted_tile.is_err());
        assert_eq!(
            converted_tile.err().unwrap(),
            ReadError::UnknownPredefinedTile {
                tile_name: "test".to_string(),
                name: "test".to_string()
            }
        );
    }

    #[test]
    fn test_convert_custom_tile() {
        let tile = Tile::Custom(vec![vec![1, 0], vec![1, 1]]);
        let converted_tile = convert_tile(&"test".to_string(), tile, &HashMap::new()).unwrap();
        let expected_tile = TileConfig::new(arr2(&[[true, false], [true, true]]));
        assert_eq!(converted_tile.base(), expected_tile.base());
    }

    #[test]
    fn test_convert_custom_tile_zero_dimension() {
        let tile = Tile::Custom(vec![]);
        let converted_tile = convert_tile(&"test".to_string(), tile, &HashMap::new());
        assert!(converted_tile.is_err());
        assert_eq!(
            converted_tile.err().unwrap(),
            ReadError::TileWidthOrHeightCannotBeZero {
                tile_name: "test".to_string(),
            }
        );

        let tile = Tile::Custom(vec![vec![1, 0], vec![]]);
        let converted_tile = convert_tile(&"test".to_string(), tile, &HashMap::new());
        assert!(converted_tile.is_err());
        assert_eq!(
            converted_tile.err().unwrap(),
            ReadError::TileWidthOrHeightCannotBeZero {
                tile_name: "test".to_string(),
            }
        );
    }
}
