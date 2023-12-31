use std::f32::consts::PI;

use super::LightFlags;
use crate::{color::RgbaSpectrum, interaction::SurfaceInteraction};
use cgmath::{InnerSpace, Point3, Vector3};

/// A point light source that emits the same amount of light in all directions.
pub struct PointLight {
    /// The light's position in world space.
    position: Point3<f32>,

    /// The amount of power emitted per unit solid angle.
    intensity: RgbaSpectrum,
}

impl PointLight {
    /// Create a new point light source.
    ///
    /// * position - The position of the light in world space.
    /// * intensity - The amount of power emitted per unit solid angle.
    pub fn new(position: Point3<f32>, intensity: RgbaSpectrum) -> Self {
        Self {
            position,
            intensity,
        }
    }

    pub fn li(&self, interaction: &SurfaceInteraction) -> (RgbaSpectrum, Vector3<f32>) {
        let light_to_point = self.position - interaction.point;
        let li = self.intensity / light_to_point.magnitude2();
        let wi = light_to_point.normalize();
        (li, wi)
    }

    pub fn power(&self) -> RgbaSpectrum {
        4.0 * PI * self.intensity
    }

    pub fn flags(&self) -> LightFlags {
        LightFlags::DELTA_POSITION
    }
}
