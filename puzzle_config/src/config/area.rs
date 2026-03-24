/// Metadata for an area on the board.
/// Includes the name and the formatter for the area values.
/// This is used by the target selection UI.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AreaConfig {
    name: String,
    formatter: AreaValueFormatter,
    default_value: String,
}

impl AreaConfig {
    pub fn new(
        name: String,
        area_value_formatter: AreaValueFormatter,
        default_value: String,
    ) -> Self {
        AreaConfig {
            name,
            formatter: area_value_formatter,
            default_value,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn formatter(&self) -> &AreaValueFormatter {
        &self.formatter
    }

    pub fn default_value(&self) -> &str {
        &self.default_value
    }
}

/// Formatter for a value for an area to display on the target selection button.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AreaValueFormatter {
    /// Displays the value as is.
    Plain,
    /// Formats the value as an ordinal number (1st, 2nd, 3rd, 4th, etc.).
    Nth,
    /// Formats the value with a prefix and suffix.
    PrefixSuffix { prefix: String, suffix: String },
}
