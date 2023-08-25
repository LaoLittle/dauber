use crate::color::Color;

#[derive(Clone, Debug)]
pub struct Paint {
    pub color: Color,
    pub style: PaintStyle,
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
    pub fn set_color<C: Into<Color>>(&mut self, color: C) {
        self.color = color.into();
    }

    #[inline]
    pub fn set_style(&mut self, style: PaintStyle) {
        self.style = style;
    }

    #[inline]
    pub fn set_anti_alias(&mut self, anti_alias: bool) {
        self.anti_alias = anti_alias;
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
