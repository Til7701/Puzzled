mod config;
mod error;
mod json;
mod validation;

pub use config::area::AreaConfig;
pub use config::area::AreaValueFormatter;
pub use config::board::BoardConfig;
pub use config::collection::PuzzleConfigCollection;
pub use config::difficulty::PuzzleDifficultyConfig;
pub use config::progression::ProgressionConfig;
pub use config::puzzle::PuzzleConfig;
pub use config::target::{Target, TargetIndex, TargetTemplate};
pub use config::tile::TileConfig;
pub use error::ReadError;
pub use json::JsonLoader;

const PUZZLED_VERSION_FIELD: &str = "puzzled";

pub fn create_json_loader(
    predefined_json_str: &str,
    puzzled_version: &str,
) -> Result<JsonLoader, ReadError> {
    let json_loader = JsonLoader::new(predefined_json_str, puzzled_version.to_string());
    Ok(json_loader)
}

#[cfg(test)]
mod tests {
    use crate::create_json_loader;
    use ndarray::arr2;

    #[test]
    fn test_load_puzzle_collection_from_json() {
        let predefined_json_str = r#"
        {
            "tiles":
                {
                  "L3": [
                    [1, 0],
                    [1, 1]
                  ]
                },
            "boards": {}
        }
        "#;
        let json_loader = create_json_loader(predefined_json_str, "0.1.0").unwrap();

        let json_str = r#"
        {
          "puzzled": "0.1.0",
          "name": "Test Collection",
          "author": "Test Author",
          "id": "de.til7701.Puzzled.test-collection",
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

        let result = json_loader.load_puzzle_collection(json_str);
        assert!(result.is_ok());
        let collection = result.unwrap();
        assert_eq!(collection.name(), "Test Collection");
        assert_eq!(collection.author(), "Test Author");
        assert_eq!(collection.id(), "de.til7701.Puzzled.test-collection");
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
