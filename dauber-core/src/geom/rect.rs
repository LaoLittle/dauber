/// A Rectangle
pub struct Rect {
    /// left
    pub l: f32,
    /// top
    pub t: f32,
    /// right
    pub r: f32,
    /// bottom
    pub b: f32,
}

impl Rect {
    #[inline]
    pub fn from_ltrb(l: f32, t: f32, r: f32, b: f32) -> Self {
        Self { l, t, r, b }
    }
}
