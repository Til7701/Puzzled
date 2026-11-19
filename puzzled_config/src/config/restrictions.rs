#[derive(Debug, Clone)]
pub struct RestrictionsConfig {
    allow_rotation: bool,
    allow_flip: bool,
}

impl RestrictionsConfig {
    pub fn new(allow_rotation: bool, allow_flip: bool) -> Self {
        RestrictionsConfig {
            allow_rotation,
            allow_flip,
        }
    }

    pub fn allow_rotation(&self) -> bool {
        self.allow_rotation
    }

    pub fn allow_flip(&self) -> bool {
        self.allow_flip
    }
}

impl Default for RestrictionsConfig {
    fn default() -> Self {
        RestrictionsConfig {
            allow_rotation: true,
            allow_flip: true,
        }
    }
}
