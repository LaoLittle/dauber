use crate::device::Device;
use crate::geom::point::Point;
use crate::paint::Paint;
use crate::path::Path;

pub struct Canvas<'a, D> {
    device: &'a mut D,
}

impl<'a, D: Device> Canvas<'a, D> {
    #[inline]
    pub(crate) fn new(device: &'a mut D) -> Self {
        Self { device }
    }

    #[inline]
    pub fn draw_path(&mut self, path: &Path, paint: &Paint) -> &mut Self {
        self.device.draw_path(path, paint);

        self
    }

    pub fn draw_circle(&mut self, center: Point, radius: f32, paint: &Paint) -> &mut Self {
        let mut path = Path::new();
        path.add_circle(center, radius);

        self.draw_path(&path, paint);

        self
    }

    #[inline]
    pub fn device(&self) -> &D {
        self.device
    }
}
