mod config;
mod error;
mod json;
pub mod random;
mod validation;

pub use config::area::AreaConfig;
pub use config::area::AreaValueFormatter;
pub use config::board::BoardConfig;
pub use config::collection::PuzzleConfigCollection;
pub use config::color::ColorConfig;
pub use config::difficulty::PuzzleDifficultyConfig;
pub use config::preview::PreviewConfig;
pub use config::progression::ProgressionConfig;
pub use config::puzzle::PuzzleConfig;
pub use config::puzzle::PuzzleId;
pub use config::target::{Target, TargetIndex, TargetTemplate};
pub use config::tile::TileConfig;
pub use error::ReadError;
pub use json::JsonLoader;

const PUZZLED_VERSION_FIELD: &str = "puzzled";

/// Create a new JsonLoader.
/// Subsequent calls to the JsonLoader's load_puzzle_collection method will check if the puzzled
/// version in the JSON matches the provided puzzled version.
/// Also, tiles and boards defined in the predefined_json_str will be available for puzzles loaded
/// via this loader.
pub fn create_json_loader(
    predefined_json_str: &str,
    puzzled_version: &str,
) -> Result<JsonLoader, ReadError> {
    let json_loader = JsonLoader::new(predefined_json_str, puzzled_version.to_string());
    Ok(json_loader)
}

pub fn get_predefined(predefined_json_str: &str, puzzled_version: &str) -> Predefined {
    json::read_predefined(predefined_json_str, puzzled_version)
}

pub struct Predefined {
    tiles: Vec<TileConfig>,
    boards: Vec<BoardConfig>,
}

impl Predefined {
    pub fn tiles(&self) -> &[TileConfig] {
        &self.tiles
    }

    pub fn boards(&self) -> &[BoardConfig] {
        &self.boards
    }
}

#[cfg(test)]
mod tests {
    use crate::create_json_loader;
    use puzzled_common::shape::shape_square;

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
            &shape_square(&[[true, true, true], [true, false, true], [true, true, true]])
        );
        let ref_tile = &puzzle.tiles()[0];
        assert_eq!(
            ref_tile.base(),
            &shape_square(&[[true, true], [false, true]])
        );
        let custom_tile = &puzzle.tiles()[1];
        assert_eq!(
            custom_tile.base(),
            &shape_square(&[[true, true], [false, true], [true, true]])
        );
    }
}
