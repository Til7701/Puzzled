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

pub fn load_puzzle_collection_from_json(
    json_str: &str,
) -> Result<PuzzleConfigCollection, ReadError> {
    let value: Value =
        serde_json::from_str(json_str).map_err(|e| ReadError::JsonError(e.to_string()))?;

    let version: Result<i32, ReadError> = match &value {
        Value::Object(object) => {
            let version_value = object.get("config_version");
            match version_value {
                Some(Value::Number(num)) => Ok(num.as_i64().unwrap_or(-1) as i32),
                _ => Err(ReadError::MalformedVersion),
            }
        }
        _ => Err(ReadError::MissingVersion),
    };
    if version? != 1 {
        return Err(ReadError::UnsupportedVersion);
    }

    json::load_puzzle_collection_from_json(value)
}
