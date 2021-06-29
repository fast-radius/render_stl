use crate::interaction::SurfaceInteraction;
use crate::ray::Ray;
use cgmath::{InnerSpace, Point3, Vector3};

pub trait Shape<'shp> {
    fn ray_intersection(&'shp self, ray: &Ray) -> Vec<(f32, SurfaceInteraction<'shp>)>;
}

#[derive(Debug)]
pub struct Sphere {}

impl<'shp> Shape<'shp> for Sphere {
    fn ray_intersection(&'shp self, ray: &Ray) -> Vec<(f32, SurfaceInteraction<'shp>)> {
        let sphere_to_ray = ray.origin - Point3::new(0.0, 0.0, 0.0);
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            vec![]
        } else {
            let t1 = (-1.0 * b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-1.0 * b + discriminant.sqrt()) / (2.0 * a);

            let si1 = SurfaceInteraction { shape: self };
            let si2 = SurfaceInteraction { shape: self };

            vec![(t1, si1), (t2, si2)]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::interaction::SurfaceInteraction;
    use crate::ray::Ray;
    use crate::shape::Sphere;
    use cgmath::{Point3, Vector3};

    use super::Shape;

    #[test]
    fn ray_intersects_at_two_points() {
        let ray = Ray {
            origin: Point3::new(0.0, 0.0, -5.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let sphere = Sphere {};
        let intersections = sphere.ray_intersection(&ray);
        assert_eq!(intersections.len(), 2);
        assert_eq!(
            intersections[0],
            (4.0, SurfaceInteraction { shape: &sphere }),
        );
        assert_eq!(
            intersections[1],
            (6.0, SurfaceInteraction { shape: &sphere }),
        );
    }

    #[test]
    fn ray_intersects_at_tangent() {
        let ray = Ray {
            origin: Point3::new(0.0, 1.0, -5.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let sphere = Sphere {};
        let intersections = sphere.ray_intersection(&ray);
        assert_eq!(intersections.len(), 2);
        assert_eq!(
            intersections[0],
            (5.0, SurfaceInteraction { shape: &sphere }),
        );
        assert_eq!(
            intersections[1],
            (5.0, SurfaceInteraction { shape: &sphere }),
        );
    }

    #[test]
    fn ray_misses() {
        let ray = Ray {
            origin: Point3::new(0.0, 2.0, -5.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let sphere = Sphere {};
        let intersections = sphere.ray_intersection(&ray);
        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let ray = Ray {
            origin: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let sphere = Sphere {};
        let intersections = sphere.ray_intersection(&ray);
        assert_eq!(intersections.len(), 2);
        assert_eq!(
            intersections[0],
            (-1.0, SurfaceInteraction { shape: &sphere }),
        );
        assert_eq!(
            intersections[1],
            (1.0, SurfaceInteraction { shape: &sphere }),
        );
    }

    #[test]
    fn sphere_is_behind_ray() {
        let ray = Ray {
            origin: Point3::new(0.0, 0.0, 5.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let sphere = Sphere {};
        let intersections = sphere.ray_intersection(&ray);
        assert_eq!(intersections.len(), 2);
        assert_eq!(
            intersections[0],
            (-6.0, SurfaceInteraction { shape: &sphere }),
        );
        assert_eq!(
            intersections[1],
            (-4.0, SurfaceInteraction { shape: &sphere }),
        );
    }
}
