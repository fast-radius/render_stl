use crate::interaction::SurfaceInteraction;
use crate::intersection::{Intersection, Intersections};
use crate::ray::Ray;
use cgmath::{InnerSpace, Matrix4, Point3, Vector3};

pub trait Shape<'shp, 'mat> {
    /// Returns a reference to the matrix that transforms the shape from object
    /// space to world space.
    fn object_to_world(&self) -> &'mat cgmath::Matrix4<f32>;

    /// Returns a reference to the matrix that transforms the shape from world
    /// space to object space.
    fn world_to_object(&self) -> &'mat cgmath::Matrix4<f32>;

    fn ray_intersections(&'shp self, ray: &Ray) -> Intersections<'shp, 'mat>;
}

#[derive(Debug)]
pub struct Sphere<'mat> {
    object_to_world: &'mat Matrix4<f32>,
    world_to_object: &'mat Matrix4<f32>,
}

impl<'mat> Sphere<'mat> {
    pub fn new(object_to_world: &'mat Matrix4<f32>, world_to_object: &'mat Matrix4<f32>) -> Self {
        Sphere {
            object_to_world,
            world_to_object,
        }
    }
}

impl<'shp, 'mat> Shape<'shp, 'mat> for Sphere<'mat> {
    fn object_to_world(&self) -> &'mat cgmath::Matrix4<f32> {
        self.object_to_world
    }

    fn world_to_object(&self) -> &'mat cgmath::Matrix4<f32> {
        self.world_to_object
    }

    fn ray_intersections(&'shp self, ray: &Ray) -> Intersections<'shp, 'mat> {
        let sphere_to_ray = ray.origin - Point3::new(0.0, 0.0, 0.0);
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            Intersections::empty()
        } else {
            let t1 = (-1.0 * b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-1.0 * b + discriminant.sqrt()) / (2.0 * a);

            let intr1 = Intersection {
                t: t1,
                interaction: SurfaceInteraction { shape: self },
            };
            let intr2 = Intersection {
                t: t2,
                interaction: SurfaceInteraction { shape: self },
            };

            Intersections::new(vec![intr1, intr2])
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::interaction::SurfaceInteraction;
    use crate::intersection::Intersection;
    use crate::matrix::identity4;
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
        let identity = identity4();
        let sphere = Sphere::new(&identity, &identity);
        let intersections = sphere.ray_intersections(&ray);
        assert_eq!(intersections.values.len(), 2);
        assert_eq!(
            intersections.values[0],
            Intersection {
                t: 4.0,
                interaction: SurfaceInteraction { shape: &sphere }
            },
        );
        assert_eq!(
            intersections.values[1],
            Intersection {
                t: 6.0,
                interaction: SurfaceInteraction { shape: &sphere }
            },
        );
    }

    #[test]
    fn ray_intersects_at_tangent() {
        let ray = Ray {
            origin: Point3::new(0.0, 1.0, -5.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let identity = identity4();
        let sphere = Sphere::new(&identity, &identity);
        let intersections = sphere.ray_intersections(&ray);
        assert_eq!(intersections.values.len(), 2);
        assert_eq!(
            intersections.values[0],
            Intersection {
                t: 5.0,
                interaction: SurfaceInteraction { shape: &sphere }
            },
        );
        assert_eq!(
            intersections.values[1],
            Intersection {
                t: 5.0,
                interaction: SurfaceInteraction { shape: &sphere }
            },
        );
    }

    #[test]
    fn ray_misses() {
        let ray = Ray {
            origin: Point3::new(0.0, 2.0, -5.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let identity = identity4();
        let sphere = Sphere::new(&identity, &identity);
        let intersections = sphere.ray_intersections(&ray);
        assert_eq!(intersections.values.len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let ray = Ray {
            origin: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let identity = identity4();
        let sphere = Sphere::new(&identity, &identity);
        let intersections = sphere.ray_intersections(&ray);
        assert_eq!(intersections.values.len(), 2);
        assert_eq!(
            intersections.values[0],
            Intersection {
                t: -1.0,
                interaction: SurfaceInteraction { shape: &sphere }
            },
        );
        assert_eq!(
            intersections.values[1],
            Intersection {
                t: 1.0,
                interaction: SurfaceInteraction { shape: &sphere }
            },
        );
    }

    #[test]
    fn sphere_is_behind_ray() {
        let ray = Ray {
            origin: Point3::new(0.0, 0.0, 5.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let identity = identity4();
        let sphere = Sphere::new(&identity, &identity);
        let intersections = sphere.ray_intersections(&ray);
        assert_eq!(intersections.values.len(), 2);
        assert_eq!(
            intersections.values[0],
            Intersection {
                t: -6.0,
                interaction: SurfaceInteraction { shape: &sphere }
            },
        );
        assert_eq!(
            intersections.values[1],
            Intersection {
                t: -4.0,
                interaction: SurfaceInteraction { shape: &sphere }
            },
        );
    }
}
