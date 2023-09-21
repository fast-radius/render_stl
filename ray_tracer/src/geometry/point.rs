use crate::geometry::axis::Axis3;
use cgmath::Point3;

/// Returns the point's component on the given axis.
pub fn component(p: Point3<f32>, axis: Axis3) -> f32 {
    match axis {
        Axis3::X => p.x,
        Axis3::Y => p.y,
        Axis3::Z => p.z,
    }
}

/// Returns a new vector whose components are taken from the components of the
/// given vector.
pub fn permute(p: Point3<f32>, new_x: Axis3, new_y: Axis3, new_z: Axis3) -> Point3<f32> {
    Point3::new(
        component(p, new_x),
        component(p, new_y),
        component(p, new_z),
    )
}

pub fn add_point3(points: Vec<Point3<f32>>) -> Point3<f32> {
    points
        .iter()
        .fold(Point3::new(0.0, 0.0, 0.0), |result, next| {
            Point3::new(result.x + next.x, result.y + next.y, result.z + next.z)
        })
}
