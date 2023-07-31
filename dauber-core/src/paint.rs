use crate::color::Color;

#[derive(Clone, Debug)]
pub struct Paint {
    color: Color,
    style: PaintStyle,
    pub anti_alias: bool,
}

impl Paint {
    #[inline]
    pub const fn new() -> Self {
        Self {
            color: Color::BLACK,
            style: PaintStyle::Fill,
            anti_alias: false,
        }
    }

    #[inline]
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    #[inline]
    pub fn set_style(&mut self, style: PaintStyle) {
        self.style = style;
    }

    pub fn style(&self) -> PaintStyle {
        self.style
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

impl Default for Paint {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum PaintStyle {
    Fill,
    Stroke(f32),
    FillAndStroke(f32),
}
