use crate::shape::prototile::Prototile;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Hexagon {
    x: u32,
    y: u32,
    z: u32,
}

impl Hexagon {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Hexagon { x, y, z }
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn z(&self) -> u32 {
        self.z
    }
}

impl Prototile for Hexagon {}
