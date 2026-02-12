use crate::config::preview::PreviewConfig;
use crate::config::progression::ProgressionConfig;
use crate::PuzzleConfig;

#[derive(Debug, Clone)]
pub struct PuzzleConfigCollection {
    name: String,
    description: Option<String>,
    author: String,
    id: String,
    version: Option<String>,
    progression: ProgressionConfig,
    preview: PreviewConfig,
    puzzles: Vec<PuzzleConfig>,
}

impl PuzzleConfigCollection {
    pub fn new(
        name: String,
        description: Option<String>,
        author: String,
        id: String,
        version: Option<String>,
        progression: ProgressionConfig,
        preview: PreviewConfig,
        puzzles: Vec<PuzzleConfig>,
    ) -> PuzzleConfigCollection {
        PuzzleConfigCollection {
            name,
            description,
            author,
            id,
            version,
            progression,
            preview,
            puzzles,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn version(&self) -> &Option<String> {
        &self.version
    }

    pub fn progression(&self) -> &ProgressionConfig {
        &self.progression
    }

    pub fn preview(&self) -> &PreviewConfig {
        &self.preview
    }

    pub fn puzzles(&self) -> &Vec<PuzzleConfig> {
        &self.puzzles
    }
}
