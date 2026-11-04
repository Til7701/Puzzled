#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct PixelPosition(pub f64, pub f64);

impl PixelPosition {
    pub fn add_tuple(&mut self, tuple: (f64, f64)) {
        self.0 += tuple.0;
        self.1 += tuple.1;
    }

    pub fn mul_scalar(&self, scalar: f64) -> PixelPosition {
        PixelPosition(self.0 * scalar, self.1 * scalar)
    }
}

impl From<(f64, f64)> for PixelPosition {
    fn from(value: (f64, f64)) -> Self {
        PixelPosition(value.0, value.1)
    }
}
