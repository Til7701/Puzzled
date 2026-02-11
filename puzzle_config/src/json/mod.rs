use crate::json::converter::Convertable;
use crate::json::model::PuzzleCollection;
use crate::json::predefined::{Custom, Predefined};
use crate::{PuzzleConfigCollection, ReadError, PUZZLED_VERSION_FIELD};
use semver::{Version, VersionReq};
use serde_json::Value;

mod converter;
mod model;
mod predefined;

pub struct JsonLoader {
    predefined: Predefined,
    version_req: VersionReq,
}

impl JsonLoader {
    pub(crate) fn new(predefined_json: &str, puzzled_version: String) -> Self {
        let predefined: Predefined =
            serde_json::from_str(predefined_json).expect("Failed to parse predefined JSON");
        Self {
            predefined,
            version_req: VersionReq::parse(format!("<={}", puzzled_version).as_str()).unwrap(),
        }
    }

    pub fn load_puzzle_collection(
        &self,
        json_str: &str,
    ) -> Result<PuzzleConfigCollection, ReadError> {
        let value: Value =
            serde_json::from_str(json_str).map_err(|e| ReadError::JsonError(e.to_string()))?;

        let version: Result<i32, ReadError> = match &value {
            Value::Object(object) => {
                let version_value = object.get(PUZZLED_VERSION_FIELD);
                match version_value {
                    Some(Value::String(s)) => {
                        let collection_version = Version::parse(s)
                            .map_err(|e| ReadError::InvalidVersion(e.to_string()))?;
                        if self.version_req.matches(&collection_version) {
                            Ok(1)
                        } else {
                            Err(ReadError::UnsupportedVersion)
                        }
                    }
                    _ => Err(ReadError::MissingVersion),
                }
            }
            _ => Err(ReadError::MissingVersion),
        };
        if version? == 1 {
            self.load(value)
        } else {
            Err(ReadError::UnsupportedVersion)
        }
    }

    fn load(&self, json_data: Value) -> Result<PuzzleConfigCollection, ReadError> {
        let result = serde_json::from_value::<PuzzleCollection>(json_data);
        match result {
            Ok(collection) => collection.convert(&self.predefined, &mut Custom::default()),
            Err(e) => Err(ReadError::JsonError(e.to_string())),
        }
    }
}
