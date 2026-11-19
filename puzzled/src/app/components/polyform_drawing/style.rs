const BORDER_SIZE_RATIO: f64 = 0.03;
const HIGHLIGHT_SIZE_RATIO: f64 = 0.08;

#[derive(Debug, Default)]
pub enum DrawingStyle {
    #[default]
    Filled,
    PrototileOutline,
    Outline,
}
