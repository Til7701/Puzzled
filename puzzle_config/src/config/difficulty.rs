#[derive(Debug, Clone, Copy)]
pub enum PuzzleDifficultyConfig {
    Easy = 1,
    Medium = 2,
    Hard = 3,
    Expert = 4,
}

impl From<PuzzleDifficultyConfig> for String {
    fn from(val: PuzzleDifficultyConfig) -> Self {
        match val {
            PuzzleDifficultyConfig::Easy => "Easy".to_string(),
            PuzzleDifficultyConfig::Medium => "Medium".to_string(),
            PuzzleDifficultyConfig::Hard => "Hard".to_string(),
            PuzzleDifficultyConfig::Expert => "Expert".to_string(),
        }
    }
}

impl From<f32> for PuzzleDifficultyConfig {
    fn from(value: f32) -> Self {
        if value <= 1.5 {
            PuzzleDifficultyConfig::Easy
        } else if value <= 2.5 {
            PuzzleDifficultyConfig::Medium
        } else if value <= 3.5 {
            PuzzleDifficultyConfig::Hard
        } else {
            PuzzleDifficultyConfig::Expert
        }
    }
}
