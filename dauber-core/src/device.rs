use crate::image_info::ImageInfo;
use crate::paint::Paint;
use crate::path::Path;

pub trait Device {
    fn new(info: ImageInfo) -> Self;

    fn image_info(&self) -> &ImageInfo;

    fn draw_path(&mut self, path: &Path, paint: &Paint);
}

struct Sink;

impl Device for Sink {
    fn new(_: ImageInfo) -> Self {
        Self
    }

    fn image_info(&self) -> &ImageInfo {
        static SINK_INFO: ImageInfo = ImageInfo::new_wh(0, 0);
        &SINK_INFO
    }

    fn draw_path(&mut self, path: &Path, paint: &Paint) {}
}
