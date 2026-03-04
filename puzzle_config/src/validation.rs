use crate::ReadError;
use once_cell::sync::Lazy;
use regex::Regex;

static COLLECTION_ID_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^([a-zA-Z0-9-]+\.)+[a-zA-Z0-9-]+$").unwrap());

pub(crate) fn validate_collection_id(id: String) -> Result<String, ReadError> {
    if id.trim().is_empty() {
        return Err(ReadError::InvalidCollectionId(id));
    }

    COLLECTION_ID_REGEX
        .find(&id)
        .ok_or(ReadError::InvalidCollectionId(id.clone()))?;

    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_collection_id_valid() {
        let valid_ids = vec![
            "com.example.collection",
            "org.dash-domain.puzzles",
            "de.til7701.numbers",
            "de.til7701.long.collection.id.with.many.parts",
        ];

        for id in valid_ids {
            assert!(
                validate_collection_id(id.to_string()).is_ok(),
                "Expected '{}' to be valid",
                id
            );
        }
    }

    #[test]
    fn test_validate_collection_id_invalid() {
        let invalid_ids = vec![
            "",
            "   ",
            "invalid id",
            "no-dots-in-this-id",
            "double..dots..id",
            ".leadingdot.collection",
            "trailingdot.collection.",
            "other@invalid#chars!.collection",
            "underscore_in_id.collection",
        ];

        for id in invalid_ids {
            assert!(
                validate_collection_id(id.to_string()).is_err(),
                "Expected '{}' to be invalid",
                id
            );
        }
    }
}
