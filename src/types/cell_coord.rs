use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellCoord {
    pub x: i32,
    pub y: i32,
}

// Instantiation
impl CellCoord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_tuple(tuple: (i32, i32)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }

    pub fn to_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

// Transformations
impl CellCoord {}

// Arithmetic
impl Add for CellCoord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<(i32, i32)> for CellCoord{
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl Sub for CellCoord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<(i32, i32)> for CellCoord {
    type Output = Self;

    fn sub(self, rhs: (i32, i32)) -> Self::Output {
        Self {
            x: self.x - rhs.0,
            y: self.y - rhs.1,
        }
    }
}
