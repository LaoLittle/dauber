#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    const WEIGHT: f32 = 1.0 / 255.0;

    #[inline]
    pub const fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 * Self::WEIGHT,
            g: g as f32 * Self::WEIGHT,
            b: b as f32 * Self::WEIGHT,
            a: a as f32 * Self::WEIGHT,
        }
    }

    pub fn to_array(self) -> [f32; 4] {
        let Self { r, g, b, a } = self;

        [r, g, b, a]
    }
}

macro_rules! color_define {
    ($($name:ident => ($r:expr, $g:expr, $b:expr, $a:expr)),* $(,)?) => {
        impl Color {
            $(pub const $name: Self = Self::from_rgba($r, $g, $b, $a);)*
        }
    };
}

color_define! {
        TRANSPARENT => (0., 0., 0., 0.),
        BLACK       => (0., 0., 0., 1.),
        DARK_GRAY   => (0.25, 0.25, 0.25, 1.),
        GRAY        => (0.50, 0.50, 0.50, 1.),
        LIGHT_GRAY  => (0.75, 0.75, 0.75, 1.),
        WHITE       => (1., 1., 1., 1.),
        RED         => (1., 0., 0., 1.),
        GREEN       => (0., 1., 0., 1.),
        BLUE        => (0., 0., 1., 1.),
        YELLOW      => (1., 1., 0., 1.),
        CYAN        => (0., 1., 1., 1.),
        MAGENTA     => (1., 0., 1., 1.)
}

macro_rules! define_withrgba {
    ($($method:ident -> $field:ident);* $(;)?) => {
        impl Color {
            $(
            pub const fn $method(self, $field: f32) -> Self {
                Self { $field, ..self }
            }
            )*
        }
    };
}

define_withrgba! {
    with_r -> r;
    with_g -> g;
    with_b -> b;
    with_a -> a;
}

impl Default for Color {
    #[inline]
    fn default() -> Self {
        Self::TRANSPARENT
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    #[inline]
    fn from((r, g, b, a): (f32, f32, f32, f32)) -> Self {
        Self::from_rgba(r, g, b, a)
    }
}

impl From<[f32; 4]> for Color {
    #[inline]
    fn from([r, g, b, a]: [f32; 4]) -> Self {
        Self::from_rgba(r, g, b, a)
    }
}
