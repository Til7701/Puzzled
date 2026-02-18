use std::ops::{Add, Sub};

/// Represents an offset in x and y directions.
///
/// The offset values are in pixels. For cell-based offsets, use `CellOffset`.
#[derive(Debug, Default, Clone, Copy)]
pub struct PixelOffset(pub f64, pub f64);

impl PixelOffset {
    pub fn add_other(&self, other: &PixelOffset) -> PixelOffset {
        Self(self.0 + other.0, self.1 + other.1)
    }

    pub fn subtract_other(&self, other: &PixelOffset) -> PixelOffset {
        Self(self.0 - other.0, self.1 - other.1)
    }

    pub fn add_tuple(&self, other: (f64, f64)) -> PixelOffset {
        Self(self.0 + other.0, self.1 + other.1)
    }

    pub fn mul_scalar(&self, scalar: f64) -> PixelOffset {
        Self(self.0 * scalar, self.1 * scalar)
    }

    pub fn div_scalar(&self, scalar: f64) -> PixelOffset {
        Self(self.0 / scalar, self.1 / scalar)
    }

    pub fn round(&self) -> PixelOffset {
        Self(self.0.round(), self.1.round())
    }
}

impl From<(f64, f64)> for PixelOffset {
    fn from(tuple: (f64, f64)) -> Self {
        PixelOffset(tuple.0, tuple.1)
    }
}

impl From<CellOffset> for PixelOffset {
    fn from(cell_offset: CellOffset) -> Self {
        PixelOffset(cell_offset.0 as f64, cell_offset.1 as f64)
    }
}

impl Add for PixelOffset {
    type Output = PixelOffset;

    fn add(self, other: PixelOffset) -> PixelOffset {
        self.add_other(&other)
    }
}

impl Sub for PixelOffset {
    type Output = PixelOffset;

    fn sub(self, other: PixelOffset) -> PixelOffset {
        self.subtract_other(&other)
    }
}

/// Represents an offset in x and y directions.
///
/// The offset values are in cell units. For pixel-based offsets, use `PixelOffset`.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct CellOffset(pub i32, pub i32);

impl CellOffset {
    pub fn add_other(&self, other: &CellOffset) -> CellOffset {
        Self(self.0 + other.0, self.1 + other.1)
    }

    pub fn subtract_other(&self, other: &CellOffset) -> CellOffset {
        Self(self.0 - other.0, self.1 - other.1)
    }

    pub fn add_tuple(&self, other: (i32, i32)) -> CellOffset {
        Self(self.0 + other.0, self.1 + other.1)
    }

    pub fn mul_scalar(&self, scalar: f64) -> CellOffset {
        Self(
            (self.0 as f64 * scalar) as i32,
            (self.1 as f64 * scalar) as i32,
        )
    }

    pub fn div_scalar(&self, scalar: f64) -> CellOffset {
        Self(
            (self.0 as f64 / scalar) as i32,
            (self.1 as f64 / scalar) as i32,
        )
    }

    pub fn max(&self, other: CellOffset) -> CellOffset {
        Self(self.0.max(other.0), self.1.max(other.1))
    }

    pub fn min(&self, other: CellOffset) -> CellOffset {
        Self(self.0.min(other.0), self.1.min(other.1))
    }
}

impl From<(i32, i32)> for CellOffset {
    fn from(tuple: (i32, i32)) -> Self {
        CellOffset(tuple.0, tuple.1)
    }
}

impl From<PixelOffset> for CellOffset {
    fn from(pixel_offset: PixelOffset) -> Self {
        CellOffset(pixel_offset.0.round() as i32, pixel_offset.1.round() as i32)
    }
}

impl From<CellOffset> for (usize, usize) {
    fn from(cell_offset: CellOffset) -> Self {
        (cell_offset.0 as usize, cell_offset.1 as usize)
    }
}

impl From<(usize, usize)> for CellOffset {
    fn from(value: (usize, usize)) -> Self {
        CellOffset(value.0 as i32, value.1 as i32)
    }
}

impl Add for CellOffset {
    type Output = CellOffset;

    fn add(self, other: CellOffset) -> CellOffset {
        self.add_other(&other)
    }
}

impl Sub for CellOffset {
    type Output = CellOffset;

    fn sub(self, other: CellOffset) -> CellOffset {
        self.subtract_other(&other)
    }
}
