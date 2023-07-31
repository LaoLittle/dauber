use crate::geom::point::Point;
use crate::geom::vector;
use crate::path::Path;

/// Tessellate the stroke for an axis-aligned rounded rectangle.
pub fn add_circle(
    path: &mut Path,
    center: Point,
    radius: f32,
    //winding: Winding,
) {
    let radius = radius.abs();
    let dir = 1.0; // negative winding: -1

    // https://spencermortensen.com/articles/bezier-circle/
    const CONSTANT_FACTOR: f32 = 0.55191505;
    let d = radius * CONSTANT_FACTOR;

    path.move_to(center + vector(-radius, 0.0));

    let ctrl_0 = center + vector(-radius, -d * dir);
    let ctrl_1 = center + vector(-d, -radius * dir);
    let mid = center + vector(0.0, -radius * dir);
    path.cubic_to(ctrl_0, ctrl_1, mid);

    let ctrl_0 = center + vector(d, -radius * dir);
    let ctrl_1 = center + vector(radius, -d * dir);
    let mid = center + vector(radius, 0.0);
    path.cubic_to(ctrl_0, ctrl_1, mid);

    let ctrl_0 = center + vector(radius, d * dir);
    let ctrl_1 = center + vector(d, radius * dir);
    let mid = center + vector(0.0, radius * dir);
    path.cubic_to(ctrl_0, ctrl_1, mid);

    let ctrl_0 = center + vector(-d, radius * dir);
    let ctrl_1 = center + vector(-radius, d * dir);
    let mid = center + vector(-radius, 0.0);
    path.cubic_to(ctrl_0, ctrl_1, mid);

    path.close();
}