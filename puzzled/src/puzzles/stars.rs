use puzzle_config::PuzzleDifficultyConfig;
use std::num::NonZeroU32;

#[derive(Debug, PartialEq, Eq)]
pub struct Stars {
    reached: u32,
    total: u32,
    max_hints_for_next_star: Option<NonZeroU32>,
}

impl Stars {
    pub fn reached(&self) -> u32 {
        self.reached
    }

    pub fn total(&self) -> u32 {
        self.total
    }

    pub fn max_hints_for_next_star(&self) -> Option<NonZeroU32> {
        self.max_hints_for_next_star
    }
}

pub fn calculate_stars(
    solved: bool,
    best_hint_count: Option<u32>,
    difficulty: &Option<PuzzleDifficultyConfig>,
) -> Stars {
    let max_stars = difficulty.map(max_stars_for_difficulty).unwrap_or(2);

    // FIXME make this better and test it
    let reached = if let Some(hint_count) = best_hint_count {
        if hint_count == 0 {
            max_stars
        } else {
            max_stars.saturating_sub(hint_count)
        }
    } else {
        0
    };

    let total = max_stars;
    let max_hints_for_next_star = if reached < total {
        Some(NonZeroU32::new(total - reached).unwrap())
    } else {
        None
    };

    Stars {
        reached,
        total,
        max_hints_for_next_star,
    }
}

const fn max_stars_for_difficulty(difficulty: PuzzleDifficultyConfig) -> u32 {
    match difficulty {
        PuzzleDifficultyConfig::Easy => 2,
        PuzzleDifficultyConfig::Medium => 3,
        PuzzleDifficultyConfig::Hard => 4,
        PuzzleDifficultyConfig::Expert => 5,
    }
}
