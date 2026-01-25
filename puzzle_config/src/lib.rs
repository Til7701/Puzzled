mod config;
mod error;
mod json;

pub use config::area::AreaConfig;
pub use config::area::AreaValueFormatter;
pub use config::board::BoardConfig;
pub use config::collection::PuzzleConfigCollection;
pub use config::puzzle::PuzzleConfig;
pub use config::target::{Target, TargetIndex, TargetTemplate};
pub use config::tile::TileConfig;
pub use error::ReadError;
use serde_json::Value;

const SPEC_VERSION_FIELD: &str = "spec";

pub fn load_puzzle_collection_from_json(
    json_str: &str,
) -> Result<PuzzleConfigCollection, ReadError> {
    let value: Value =
        serde_json::from_str(json_str).map_err(|e| ReadError::JsonError(e.to_string()))?;

    let version: Result<i32, ReadError> = match &value {
        Value::Object(object) => {
            let version_value = object.get(SPEC_VERSION_FIELD);
            match version_value {
                Some(Value::Number(num)) => Ok(num.as_i64().unwrap_or(-1) as i32),
                _ => Err(ReadError::MissingVersion),
            }
        }
        _ => Err(ReadError::MissingVersion),
    };
    if version? == 1 {
        json::load_puzzle_collection_from_json(value)
    } else {
        Err(ReadError::UnsupportedVersion)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;
    #[test]
    fn test_load_puzzle_collection_from_json() {
        let json_str = r#"
        {
          "spec": 1,
          "name": "Test Collection",
          "author": "Test Author",
          "description": "A test puzzle collection",
          "custom_tiles": {
            "testTile": [
              [1, 0, 1],
              [1, 1, 1]
            ]
          },
          "custom_boards": {
            "3x3": {
              "layout": [
                [0, 0, 0],
                [0, 1, 0],
                [0, 0, 0]
              ]
            }
          },
          "puzzles": [
            {
              "name": "Simple",
              "tiles": [
                "L3",
                "testTile"
              ],
              "board": "3x3"
            }
          ]
        }
        "#;

        let result = load_puzzle_collection_from_json(json_str);
        assert!(result.is_ok());
        let collection = result.unwrap();
        assert_eq!(collection.name(), "Test Collection");
        assert_eq!(collection.author(), "Test Author");
        assert_eq!(
            collection.description(),
            &Some("A test puzzle collection".to_string())
        );
        assert_eq!(1, collection.puzzles().len());
        let puzzle = &collection.puzzles()[0];
        assert_eq!(puzzle.name(), "Simple");
        assert_eq!(2, puzzle.tiles().len());
        assert_eq!(
            puzzle.board_config().layout(),
            arr2(&[[true, true, true], [true, false, true], [true, true, true]])
        );
        let ref_tile = &puzzle.tiles()[0];
        assert_eq!(
            ref_tile.base(),
            arr2(&[[true, false], [true, true]]).reversed_axes()
        );
        let custom_tile = &puzzle.tiles()[1];
        assert_eq!(
            custom_tile.base(),
            arr2(&[[true, false, true], [true, true, true]]).reversed_axes()
        );
    }
}
