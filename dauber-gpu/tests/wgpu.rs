use dauber_core::color::Color;
use dauber_core::device::Device;
use dauber_core::geom::point;
use dauber_core::geom::point::Point;
use dauber_core::image_info::ImageInfo;
use dauber_core::paint::{Paint, PaintStyle};
use dauber_core::path::Path;
use dauber_gpu::device::Wgpu;
use std::fs::write;

#[test]
fn wgpu() {
    let mut wgpu = Wgpu::new(ImageInfo::new_wh(1024, 512));
    let mut path = Path::new();
    let mut paint = Paint::new();
    paint.set_style(PaintStyle::Fill);
    paint.set_color(Color::GREEN);
    paint.anti_alias = true;

    path.move_to(Point::new(0., 0.));
    path.line_to(Point::new(100., 50.));
    path.line_to(Point::new(50., 200.));
    path.line_to(Point::new(200., 200.));
    path.close();

    wgpu.clear(Color::BLACK);
    wgpu.draw_path(&path, &paint);

    let mut path = Path::new();
    paint.anti_alias = false;
    paint.set_color(Color::CYAN);
    path.add_circle(Point::new(500., 200.), 100.);

    wgpu.draw_path(&path, &paint);

    let mut path = Path::new();
    paint.set_style(PaintStyle::Stroke(4.));
    paint.set_color(Color::RED);
    paint.anti_alias = true;

    path.move_to(point(650., 100.));
    path.cubic_to(point(850., 400.), point(900., 0.), point(1000., 100.));

    wgpu.draw_path(&path, &paint);

    let v = wgpu.encode_to_png();

    write("out.png", v).unwrap();
}
