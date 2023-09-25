/// A configuration that describes how to render a mesh.
#[derive(Debug, Clone)]
pub struct Config {
    pub(super) width: usize,
    pub(super) height: usize,
    pub(super) crop: bool,
    pub(super) sampler: Sampler,
    pub(super) lights: Vec<Light>,
    pub(super) camera: Camera,
    pub(super) material: Material,

    /// Indicates whether the vertex positions in the mesh assume a right hand
    /// coordinate system or a left hand coordinate system.
    pub(super) handedness: Handedness,
}

impl Config {
    /// Returns a configuration for a rendering using a left-handed coordinate
    /// system and a default camera and material.
    pub fn new_left_handed(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            crop: false,
            sampler: Sampler::default(),
            lights: vec![],
            camera: Camera::default(),
            material: Material::default(),
            handedness: Handedness::LeftHanded,
        }
    }

    /// Returns a configuration for a rendering using a right-handed coordinate
    /// system and a default camera and material.
    pub fn new_right_handed(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            crop: false,
            sampler: Sampler::default(),
            lights: vec![],
            camera: Camera::default(),
            material: Material::default(),
            handedness: Handedness::RightHanded,
        }
    }

    /// Adds a new point light to the configuration.
    pub fn point_light(
        mut self,
        r: f32,
        g: f32,
        b: f32,
        radius: f32,
        theta: f32,
        phi: f32,
    ) -> Self {
        let light = Light::PointLight {
            position: Spherical { radius, theta, phi },
            intensity: Rgb { r, g, b },
        };
        self.lights.push(light);
        self
    }

    /// Updates the camera in the configuration to a new orthographic camera.
    pub fn orthographic_camera(
        mut self,
        z_near: f32,
        z_far: f32,
        radius: f32,
        theta: f32,
        phi: f32,
    ) -> Self {
        let camera = Camera::OrthographicCamera {
            position: Spherical { radius, theta, phi },
            z_near,
            z_far,
        };
        self.camera = camera;
        self
    }

    /// Updates the configuration to crop transparent pixels from the edges of
    /// the rendering.
    ///
    /// This image produced by the renderer will be smaller than the width and
    /// height specified in the configuration if transparent pixels are cropped
    /// from the edges.
    pub fn crop_transparent(mut self) -> Self {
        self.crop = true;
        self
    }

    /// Updates the material used to render the mesh.
    #[allow(clippy::too_many_arguments)]
    pub fn material(
        mut self,
        r: f32,
        g: f32,
        b: f32,
        ambient: f32,
        diffuse: f32,
        specular: f32,
        shininess: f32,
    ) -> Self {
        let color = Rgb { r, g, b };
        let material = Material {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        };
        self.material = material;
        self
    }
}

/// A position in spherical coordinates.
#[derive(Debug, Clone, Copy)]
pub(super) struct Spherical {
    pub radius: f32,
    pub theta: f32,
    pub phi: f32,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Debug, Clone)]
pub(super) enum Sampler {
    StratifiedSampler {
        x_strata_count: usize,
        y_strata_count: usize,
        jitter: bool,
    },
}

impl Default for Sampler {
    fn default() -> Self {
        Self::StratifiedSampler {
            x_strata_count: 2,
            y_strata_count: 2,
            jitter: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Handedness {
    LeftHanded,
    RightHanded,
}

#[derive(Debug, Clone)]
pub(super) struct Material {
    pub color: Rgb,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Rgb {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
            ambient: 0.05,
            diffuse: 0.7,
            specular: 0.0,
            shininess: 80.0,
        }
    }
}

#[derive(Debug, Clone)]
pub(super) enum Light {
    /// A point light source that emits the same amount of light in all directions.
    PointLight {
        position: Spherical,

        /// The amount of power emitted per unit solid angle.
        intensity: Rgb,
    },
}

#[derive(Debug, Clone)]
pub(super) enum Camera {
    OrthographicCamera {
        position: Spherical,

        /// Distance between the near clipping plane and the camera.
        z_near: f32,

        /// Distance between the far clipping plane and the camera.
        z_far: f32,
    },
}

impl Default for Camera {
    fn default() -> Self {
        Self::OrthographicCamera {
            position: Spherical {
                radius: 1.0,
                theta: 0.0,
                phi: 0.0,
            },
            z_near: 0.0,
            z_far: 10.0,
        }
    }
}
