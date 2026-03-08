use puzzle_config::PuzzleDifficultyConfig;

/// A lookup table to find out how many stars are awarded for a given difficulty and best hint count.
///
/// This array gives the maximum number of hints that can be used to achieve a certain number of stars for each difficulty level.
/// The first entry is used, when no difficulty is set for the puzzle.
/// The following entries are used for the respective difficulties in the order of the [PuzzleDifficultyConfig].
const STARS_LOOKUP: [StarsLookup; 5] = [
    // No difficulty
    StarsLookup {
        max_hints_used_for_stars: &[u32::MAX, 0],
    },
    // Easy
    StarsLookup {
        max_hints_used_for_stars: &[u32::MAX, 2, 1, 1, 0],
    },
    // Medium
    StarsLookup {
        max_hints_used_for_stars: &[u32::MAX, 3, 2, 1, 0],
    },
    // Hard
    StarsLookup {
        max_hints_used_for_stars: &[u32::MAX, 4, 3, 2, 0],
    },
    // Expert
    StarsLookup {
        max_hints_used_for_stars: &[u32::MAX, 6, 4, 3, 0],
    },
];

#[derive(Debug, PartialEq, Eq)]
struct StarsLookup {
    max_hints_used_for_stars: &'static [u32],
}

/// Represents the stars achieved for a puzzle based on the best hint count and difficulty.
#[derive(Debug, PartialEq, Eq)]
pub struct Stars {
    reached: u32,
    total: u32,
    /// Must be `Some`, if `0 < reached < total`, and `None` otherwise.
    max_hints_for_next_star: Option<u32>,
    best_hint_count: Option<u32>,
}

impl Stars {
    /// Returns the number of stars reached.
    pub fn reached(&self) -> u32 {
        self.reached
    }

    /// Returns the total number of stars available for the puzzle.
    pub fn total(&self) -> u32 {
        self.total
    }

    /// Returns the maximum number of hints that can be used to achieve the next star.
    ///
    /// Is None if all stars have been reached or if the puzzle has not been solved yet, indicating
    /// that any solution gives the first star.
    pub fn max_hints_for_next_star(&self) -> Option<u32> {
        self.max_hints_for_next_star
    }

    /// Returns a message to be shown as a description of the current progress and how to get more
    /// stars.
    ///
    /// This may be none under some conditions.
    pub fn message(&self) -> Option<String> {
        if self.reached() == self.total() {
            None
        } else if self.reached() == 0 {
            Some("Get the first star by solving the puzzle.".to_string())
        } else {
            let max = self.max_hints_for_next_star().unwrap_or(u32::MAX);
            if max == 0 {
                let message =
                    "Solve the puzzle without using hints to get the next star.".to_string();
                Some(self.add_best_hint_count(message))
            } else {
                let message = format!("Use at most {} hints to get the next star.", max);
                Some(self.add_best_hint_count(message))
            }
        }
    }

    fn add_best_hint_count(&self, message: String) -> String {
        if let Some(best) = self.best_hint_count {
            format!("{} Best: {}", message, best)
        } else {
            message
        }
    }
}

pub fn calculate_stars(
    solved: bool,
    best_hint_count: Option<u32>,
    difficulty: &Option<PuzzleDifficultyConfig>,
) -> Stars {
    let stars_lookup = stars_lookup_for_difficulty(difficulty);
    let total = stars_lookup.max_hints_used_for_stars.len() as u32;

    if !solved {
        return Stars {
            reached: 0,
            total,
            max_hints_for_next_star: None,
            best_hint_count,
        };
    }

    let best_hint_count = best_hint_count.unwrap_or(u32::MAX);
    let mut reached = 0;
    let mut max_hints_for_next_star = None;

    for &max_hints in stars_lookup.max_hints_used_for_stars {
        if best_hint_count <= max_hints {
            reached += 1;
        } else {
            max_hints_for_next_star = Some(max_hints);
            break;
        }
    }

    Stars {
        reached,
        total,
        max_hints_for_next_star,
        best_hint_count: None,
    }
}

fn stars_lookup_for_difficulty(
    difficulty: &Option<PuzzleDifficultyConfig>,
) -> &'static StarsLookup {
    let difficulty_index = difficulty.map(|d| d as usize).unwrap_or(0);
    &STARS_LOOKUP[difficulty_index]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stars_lookup_for_difficulty() {
        assert_eq!(stars_lookup_for_difficulty(&None), &STARS_LOOKUP[0]);
        assert_eq!(
            stars_lookup_for_difficulty(&Some(PuzzleDifficultyConfig::Easy)),
            &STARS_LOOKUP[1]
        );
        assert_eq!(
            stars_lookup_for_difficulty(&Some(PuzzleDifficultyConfig::Medium)),
            &STARS_LOOKUP[2]
        );
        assert_eq!(
            stars_lookup_for_difficulty(&Some(PuzzleDifficultyConfig::Hard)),
            &STARS_LOOKUP[3]
        );
        assert_eq!(
            stars_lookup_for_difficulty(&Some(PuzzleDifficultyConfig::Expert)),
            &STARS_LOOKUP[4]
        );
    }

    #[test]
    fn test_calculate_stars_false_none_none() {
        let stars = calculate_stars(false, None, &None);
        assert_eq!(stars.reached(), 0);
        assert_eq!(stars.total(), 2);
        assert_eq!(stars.max_hints_for_next_star(), None);
    }

    #[test]
    fn test_calculate_stars_false_0_none() {
        let stars = calculate_stars(false, Some(0), &None);
        assert_eq!(stars.reached(), 0);
        assert_eq!(stars.total(), 2);
        assert_eq!(stars.max_hints_for_next_star(), None);
    }

    #[test]
    fn test_calculate_stars_false_1_none() {
        let stars = calculate_stars(false, Some(1), &None);
        assert_eq!(stars.reached(), 0);
        assert_eq!(stars.total(), 2);
        assert_eq!(stars.max_hints_for_next_star(), None);
    }

    #[test]
    fn test_calculate_stars_true_none_none() {
        let stars = calculate_stars(true, None, &None);
        assert_eq!(stars.reached(), 1);
        assert_eq!(stars.total(), 2);
        assert_eq!(stars.max_hints_for_next_star(), Some(0));
    }

    #[test]
    fn test_calculate_stars_true_0_none() {
        let stars = calculate_stars(true, Some(0), &None);
        assert_eq!(stars.reached(), 2);
        assert_eq!(stars.total(), 2);
        assert_eq!(stars.max_hints_for_next_star(), None);
    }

    #[test]
    fn test_calculate_stars_true_1_none() {
        let stars = calculate_stars(true, Some(1), &None);
        assert_eq!(stars.reached(), 1);
        assert_eq!(stars.total(), 2);
        assert_eq!(stars.max_hints_for_next_star(), Some(0));
    }

    #[test]
    fn test_calculate_stars_false_none_medium() {
        let stars = calculate_stars(false, None, &Some(PuzzleDifficultyConfig::Medium));
        assert_eq!(stars.reached(), 0);
        assert_eq!(stars.total(), 5);
        assert_eq!(stars.max_hints_for_next_star(), None);
    }

    #[test]
    fn test_calculate_stars_false_0_medium() {
        let stars = calculate_stars(false, Some(0), &Some(PuzzleDifficultyConfig::Medium));
        assert_eq!(stars.reached(), 0);
        assert_eq!(stars.total(), 5);
        assert_eq!(stars.max_hints_for_next_star(), None);
    }

    #[test]
    fn test_calculate_stars_false_1_medium() {
        let stars = calculate_stars(false, Some(1), &Some(PuzzleDifficultyConfig::Medium));
        assert_eq!(stars.reached(), 0);
        assert_eq!(stars.total(), 5);
        assert_eq!(stars.max_hints_for_next_star(), None);
    }

    #[test]
    fn test_calculate_stars_true_none_medium() {
        let stars = calculate_stars(true, None, &Some(PuzzleDifficultyConfig::Medium));
        assert_eq!(stars.reached(), 1);
        assert_eq!(stars.total(), 5);
        assert_eq!(stars.max_hints_for_next_star(), Some(3));
    }

    #[test]
    fn test_calculate_stars_true_0_medium() {
        let stars = calculate_stars(true, Some(0), &Some(PuzzleDifficultyConfig::Medium));
        assert_eq!(stars.reached(), 5);
        assert_eq!(stars.total(), 5);
        assert_eq!(stars.max_hints_for_next_star(), None);
    }

    #[test]
    fn test_calculate_stars_true_1_medium() {
        let stars = calculate_stars(true, Some(1), &Some(PuzzleDifficultyConfig::Medium));
        assert_eq!(stars.reached(), 4);
        assert_eq!(stars.total(), 5);
        assert_eq!(stars.max_hints_for_next_star(), Some(0));
    }

    #[test]
    fn test_calculate_stars_true_2_medium() {
        let stars = calculate_stars(true, Some(2), &Some(PuzzleDifficultyConfig::Medium));
        assert_eq!(stars.reached(), 3);
        assert_eq!(stars.total(), 5);
        assert_eq!(stars.max_hints_for_next_star(), Some(1));
    }

    #[test]
    fn test_calculate_stars_true_3_medium() {
        let stars = calculate_stars(true, Some(3), &Some(PuzzleDifficultyConfig::Medium));
        assert_eq!(stars.reached(), 2);
        assert_eq!(stars.total(), 5);
        assert_eq!(stars.max_hints_for_next_star(), Some(2));
    }
}
