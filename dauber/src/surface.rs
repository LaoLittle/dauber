use crate::canvas::Canvas;
use crate::device::Device;

pub struct Surface<D> {
    device: D,
}

pub type DynSurface = Surface<Box<dyn Device>>;

impl<D: Device> Surface<D> {
    #[inline]
    pub fn width(&self) -> u32 {
        self.device.image_info().width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.device.image_info().width
    }

    #[inline]
    pub fn canvas(&mut self) -> Canvas<D> {
        Canvas::new(&mut self.device)
    }

    #[inline]
    pub fn device(&self) -> &D {
        &self.device
    }
}
