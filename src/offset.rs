use std::ops::Add;

/// Represents an offset in x and y directions.
///
/// The offset can be in pixels or in grid cells, depending on the context.
#[derive(Debug, Default, Clone, Copy)]
pub struct Offset {
    pub x: f64,
    pub y: f64,
}

impl Offset {
    pub fn new(x: f64, y: f64) -> Self {
        Offset { x, y }
    }

    pub fn add_other(&self, other: &Offset) -> Offset {
        Self::new(self.x + other.x, self.y + other.y)
    }

    pub fn add_tuple(&self, other: (f64, f64)) -> Offset {
        Self::new(self.x + other.0, self.y + other.1)
    }

    pub fn mul_scalar(&self, scalar: f64) -> Offset {
        Self::new(self.x * scalar, self.y * scalar)
    }

    pub fn div_scalar(&self, scalar: f64) -> Offset {
        Self::new(self.x / scalar, self.y / scalar)
    }

    pub fn round(&self) -> Offset {
        Self::new(self.x.round(), self.y.round())
    }
}

impl Add for Offset {
    type Output = Offset;

    fn add(self, other: Offset) -> Offset {
        self.add_other(&other)
    }
}
