use crate::geom::vector::Vector;
use std::ops::Add;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Add<Vector<f32>> for Point {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Vector<f32>) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl From<(f32, f32)> for Point {
    #[inline]
    fn from((x, y): (f32, f32)) -> Self {
        Self::new(x, y)
    }
}

impl From<[f32; 2]> for Point {
    #[inline]
    fn from([x, y]: [f32; 2]) -> Self {
        Self::new(x, y)
    }
}

impl From<Vector<f32>> for Point {
    #[inline]
    fn from(Vector { x, y }: Vector<f32>) -> Self {
        Self { x, y }
    }
}
