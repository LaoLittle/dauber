#[derive(Copy, Clone, Debug)]
pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
    // color type?
}

impl ImageInfo {
    #[inline]
    pub const fn new_wh(w: u32, h: u32) -> Self {
        Self {
            width: w,
            height: h,
        }
    }
}
