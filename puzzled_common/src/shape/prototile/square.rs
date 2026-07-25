use crate::shape::prototile::Prototile;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Square {
    x: u32,
    y: u32,
}

impl Square {
    pub fn new(x: u32, y: u32) -> Self {
        Square { x, y }
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn set_x(&mut self, x: u32) {
        self.x = x;
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn set_y(&mut self, y: u32) {
        self.y = y;
    }

    pub fn rotate_counterclockwise(&mut self, viewport: (u32, u32)) {
        let old_x = self.x;
        let old_y = self.y;
        self.x = old_y;
        self.y = viewport.1 - old_x;
    }

    pub fn flip_default(&mut self, viewport: (u32, u32)) {
        let old_x = self.x;
        let old_y = self.y;
        self.x = viewport.0 - old_x;
        self.y = old_y;
    }

    pub fn transpose(&mut self, _: (u32, u32)) {
        let old_x = self.x;
        let old_y = self.y;
        self.x = old_x;
        self.y = old_y;
    }
}

impl Prototile for Square {}
