use cgmath::{Matrix4, Point3, Transform, Vector3};

#[derive(Debug)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,

    /// The upper bound of t in the ray's parametric equation,
    /// r(t) = o + t*d, 0 < t < time_max
    /// Limits the ray to a finite segment.
    pub t_max: f32,
}

impl Ray {
    pub fn new(origin: Point3<f32>, direction: Vector3<f32>, t_max: f32) -> Self {
        Self {
            origin,
            direction,
            t_max,
        }
    }

    /// Get the position along the ray for a given parametric value, `t`.
    pub fn at_t(&self, t: f32) -> Point3<f32> {
        self.origin + self.direction * t
    }
}

impl From<&Ray> for bvh::ray::Ray {
    fn from(val: &Ray) -> Self {
        bvh::ray::Ray::new(
            bvh::Point3::new(val.origin.x, val.origin.y, val.origin.z),
            bvh::Vector3::new(val.direction.x, val.direction.y, val.direction.z),
        )
    }
}

impl crate::geometry::Transform<Ray> for Matrix4<f32> {
    fn transform(&self, ray: &Ray) -> Ray {
        Ray {
            origin: self.transform_point(ray.origin),
            // It's important to leave direction unnormalized so that the ray
            // can shink or grow when we apply transformations that are intended
            // to scale an object.
            direction: self.transform_vector(ray.direction),
            t_max: ray.t_max,
        }
    }
}

/// Contains the origin and direction of two auxilary rays for some primary ray.
/// The auxilary rays are offset from the primary in the x and y directions,
/// respectively, on the film plane.
#[derive(Debug)]
pub struct RayDifferential {
    /// Origin of a ray that is offset from some primary ray in the x direction
    /// on the film plane.
    pub dx_origin: cgmath::Point3<f32>,

    /// Direction of a ray that is offset from some primary ray in the x
    /// direction on the film plane.
    pub dx_direction: cgmath::Vector3<f32>,

    /// Origin of a ray that is offset from some primary ray in the y direction
    /// on the film plane.
    pub dy_origin: cgmath::Point3<f32>,

    /// Direction of a ray that is offset from some primary ray in the x
    /// direction on the film plane.
    pub dy_direction: cgmath::Vector3<f32>,
}

impl RayDifferential {
    pub fn new(
        dx_origin: Point3<f32>,
        dx_direction: Vector3<f32>,
        dy_origin: Point3<f32>,
        dy_direction: Vector3<f32>,
    ) -> Self {
        Self {
            dx_origin,
            dx_direction,
            dy_origin,
            dy_direction,
        }
    }
}

impl crate::geometry::transform::Transform<RayDifferential> for Matrix4<f32> {
    fn transform(&self, rd: &RayDifferential) -> RayDifferential {
        RayDifferential {
            dx_origin: self.transform_point(rd.dx_origin),
            dx_direction: self.transform_vector(rd.dx_direction),
            dy_origin: self.transform_point(rd.dy_origin),
            dy_direction: self.transform_vector(rd.dy_direction),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Ray;
    use crate::{geometry::Transform, test::ApproxEq};
    use cgmath::{Matrix4, Point3, Vector3};

    #[test]
    fn at_t() {
        let ray = Ray {
            origin: Point3::new(2.0, 3.0, 4.0),
            direction: Vector3::new(1.0, 0.0, 0.0),
            t_max: f32::MAX,
        };
        assert!(ray.at_t(0.0).approx_eq(&Point3::new(2.0, 3.0, 4.0)));
        assert!(ray.at_t(1.0).approx_eq(&Point3::new(3.0, 3.0, 4.0)));
        assert!(ray.at_t(-1.0).approx_eq(&Point3::new(1.0, 3.0, 4.0)));
        assert!(ray.at_t(2.5).approx_eq(&Point3::new(4.5, 3.0, 4.0)));
    }

    #[test]
    fn translating() {
        let ray = Ray {
            origin: Point3::new(1.0, 2.0, 3.0),
            direction: Vector3::new(0.0, 1.0, 0.0),
            t_max: f32::MAX,
        };
        let t: Matrix4<f32> = Matrix4::from_translation(Vector3::new(3.0, 4.0, 5.0));
        let ray = t.transform(&ray);
        assert!(ray.origin.approx_eq(&Point3::new(4.0, 6.0, 8.0)));
        assert!(ray.direction.approx_eq(&Vector3::new(0.0, 1.0, 0.0)));
    }

    #[test]
    fn scaling() {
        let ray = Ray {
            origin: Point3::new(1.0, 2.0, 3.0),
            direction: Vector3::new(0.0, 1.0, 0.0),
            t_max: f32::MAX,
        };
        let t: Matrix4<f32> = Matrix4::from_nonuniform_scale(2.0, 3.0, 4.0);
        let ray = t.transform(&ray);
        assert!(ray.origin.approx_eq(&Point3::new(2.0, 6.0, 12.0)));
        assert!(ray.direction.approx_eq(&Vector3::new(0.0, 3.0, 0.0)));
    }
}
