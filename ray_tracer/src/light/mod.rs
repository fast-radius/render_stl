mod point;

use self::point::PointLight;
use crate::{color::RgbaSpectrum, interaction::SurfaceInteraction};
use bitflags::bitflags;
use cgmath::{Point2, Point3, Vector3};

pub enum Light {
    PointLight(PointLight),
}

impl Light {
    pub fn point_light(position: Point3<f32>, intensity: RgbaSpectrum) -> Self {
        Self::PointLight(PointLight::new(position, intensity))
    }

    /// Given a surface interation containing a point and a time, return the
    /// radiance arriving at that point and time due to the light source,
    /// ignoring possible occlusion. In addition to incoming radiance, this
    /// method also returns the incident direction from the surface point to the
    /// light source, and a visibility tester.
    // TODO: Maybe rename to `incident_light`.
    pub fn li(&self, interaction: &SurfaceInteraction) -> (RgbaSpectrum, Vector3<f32>) {
        match self {
            Light::PointLight(pl) => pl.li(interaction),
        }
    }

    // TODO: See p. 716 for explanation.
    pub fn sample_li(
        &self,
        interaction: &SurfaceInteraction,
        _u: &Point2<f32>,
    ) -> (RgbaSpectrum, Vector3<f32>, f32) {
        let (li, wi) = self.li(interaction);
        (li, wi, 1.0)
    }

    /// Return an approximation of the light's total emitted power.
    ///
    /// This is useful for light transport algorithms that will spend more time
    /// sampling and modeling lights that emit more power.
    pub fn power(&self) -> RgbaSpectrum {
        match self {
            Light::PointLight(pl) => pl.power(),
        }
    }

    /// Returns the light flags that describe the type of light source.
    pub fn flags(&self) -> LightFlags {
        match self {
            Light::PointLight(pl) => pl.flags(),
        }
    }
}

bitflags! {
    /// A bit flag representing the different types of bidirectional relectance
    /// or transmittance distribution functions.
    pub struct LightFlags: u8 {
        /// The light source's position is represented by a delta distribution.
        /// (e.g., a point light)
        const DELTA_POSITION = 0b00000001;

        /// The light source's direction is represented by a delta distribution.
        /// (e.g., a directional light)
        const DELTA_DIRECTION = 0b00000010;

        const AREA = 0b00000100;

        const INFINITE = 0b00001000;
    }
}
