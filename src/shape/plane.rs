use crate::interaction::SurfaceInteraction;
use crate::ray::Ray;
use cgmath::{InnerSpace, Matrix, Matrix4, Transform, Vector3};

/// Returns the plane's normal in world space.
pub fn normal(world_to_object: &Matrix4<f32>, reverse_orientation: bool) -> Vector3<f32> {
    let obj_n = Vector3::new(0.0, 1.0, 0.0);
    let n = world_to_object
        .transpose()
        .transform_vector(obj_n)
        .normalize();
    if reverse_orientation {
        n * 1.0
    } else {
        n
    }
}

/// Returns intersections between the ray and the plane in world space.
pub fn ray_intersections(
    ray: &Ray,
    object_to_world: &Matrix4<f32>,
    world_to_object: &Matrix4<f32>,
    reverse_orientation: bool,
) -> Vec<(f32, SurfaceInteraction)> {
    // Transforming the ray from world to object space is analagous to
    // transforming the sphere from object to world space.
    use crate::transform::Transform;
    let obj_ray = world_to_object.transform(ray);

    if obj_ray.direction.y.abs() < 0.0001 {
        vec![]
    } else {
        let t = -1.0 * ray.origin.y / ray.direction.y;
        let obj_p = obj_ray.at_t(t);
        let world_p = object_to_world.transform_point(obj_p);
        let intr = (
            t,
            SurfaceInteraction {
                point: world_p,
                neg_ray_direction: -1.0 * ray.direction,
                normal: normal(world_to_object, reverse_orientation),
            },
        );
        vec![intr]
    }
}
