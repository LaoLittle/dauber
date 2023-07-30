use crate::device::Device;

pub struct Canvas<'a, D> {
    device: &'a mut D,
}

impl<'a, D: Device> Canvas<'a, D> {
    #[inline]
    pub(crate) fn new(device: &'a mut D) -> Self {
        Self { device }
    }

    pub fn draw_path(&mut self) -> &mut Self {
        // todo: Device::draw_path()
        self
    }

    pub fn draw_circle(&mut self) -> &mut Self {
        self
    }

    #[inline]
    pub fn device(&self) -> &D {
        self.device
    }
}
