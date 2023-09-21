use crate::{
    color::RgbaSpectrum, geometry::vector, integrator::RayTracer, interaction::SurfaceInteraction,
    light::Light, ray::Ray, sampler::IncrementalSampler,
};
use cgmath::InnerSpace;

use super::{Material, Scene};

pub struct OriginalRayTracer {}

impl<'msh, Sampler: IncrementalSampler> RayTracer<Scene<'msh>, Sampler> for OriginalRayTracer {
    fn incoming_radiance(
        &self,
        ray: &Ray,
        scene: &Scene<'msh>,
        _sampler: &mut Sampler,
        _depth: usize,
        _max_depth: usize,
    ) -> RgbaSpectrum {
        Self::color_at(scene, ray)
    }
}

impl OriginalRayTracer {
    pub fn color_at(scene: &Scene, ray: &Ray) -> RgbaSpectrum {
        if let Some((_t, primitive, interaction)) = scene.primitives.ray_intersection(ray) {
            Self::shade_surface_interaction(scene, &interaction, &primitive.material)
        } else {
            RgbaSpectrum::transparent()
        }
    }

    pub fn shade_surface_interaction(
        scene: &Scene,
        interaction: &SurfaceInteraction,
        material: &Material,
    ) -> RgbaSpectrum {
        scene
            .lights
            .iter()
            .fold(RgbaSpectrum::constant(0.0), |color, light| {
                let surface = Self::shading(material, light, interaction);
                color + surface
            })
    }

    fn shading(
        material: &Material,
        light: &Light, // FIXME
        interaction: &SurfaceInteraction,
    ) -> RgbaSpectrum {
        let (incident_light, to_light) = light.li(interaction);
        let effective_color = material.color * incident_light;
        let ambient = effective_color * material.ambient;

        // light_dot_normal is the cosine of the angle between the light and normal.
        // If it's negative then the light is on the other side of the surface.
        let light_dot_normal = to_light.dot(interaction.original_geometry.normal);

        let (diffuse, specular) = if light_dot_normal >= 0.0 {
            let diffuse = effective_color * material.diffuse * light_dot_normal;

            // reflect_dot_eye is the cosine of the angle between the reflection and
            // the camera. If it's negative then the reflection is not visible.
            let reflect = vector::reflect(-1.0 * to_light, interaction.original_geometry.normal);
            let reflect_dot_eye = reflect.dot(interaction.neg_ray_direction);
            let specular = if reflect_dot_eye <= 0.0 {
                RgbaSpectrum::black()
            } else {
                let factor = reflect_dot_eye.powf(material.shininess);
                incident_light * material.specular * factor
            };

            (diffuse, specular)
        } else {
            (RgbaSpectrum::black(), RgbaSpectrum::black())
        };

        ambient + diffuse + specular
    }
}
