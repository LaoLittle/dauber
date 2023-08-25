pub mod point;
pub mod rect;
pub mod vector;

#[inline]
pub const fn point(x: f32, y: f32) -> point::Point {
    point::Point::new(x, y)
}

#[inline]
pub const fn vector<T>(x: T, y: T) -> vector::Vector<T> {
    vector::Vector::new(x, y)
}
