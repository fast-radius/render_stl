mod config;
mod error;

use cgmath::{
    point2, point3, vec2, Deg, InnerSpace, Matrix4, Point2, Point3, Rad, Transform, Vector2,
    Vector3,
};
use config::Handedness;
use image::{imageops, ImageBuffer, ImageOutputFormat, Rgba};
use mesh::{Mesh, MeshBuilder};
use ray_tracer::color::RgbaSpectrum;
use ray_tracer::filter::MitchellFilter;
use ray_tracer::light::Light;
use ray_tracer::sampler::StratifiedSampler;
use ray_tracer::simple::{Material, OriginalRayTracer, PrimitiveAggregate, Scene};
use ray_tracer::{camera::OrthographicCamera, film::Film};
use std::cmp;
use std::f32::consts::{FRAC_PI_2, PI};
use std::io::{Read, Seek, Write};
use typed_arena::Arena;

pub use config::Config;
pub use error::Error;

/// Renders the given STL file to an image.
pub fn render_to_image<R: Read + Seek>(
    stl_file: R,
    config: &Config,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, Error> {
    let mut mesh_arena = Arena::new();
    let mesh = load_mesh(stl_file, &mut mesh_arena, config.handedness)?;
    let material = load_material(&config.material);
    let lights = config.lights.iter().map(load_light).collect();
    let scene = Scene::new(
        PrimitiveAggregate::Vector(vec![PrimitiveAggregate::from_mesh(mesh, material)]),
        lights,
    );

    let resolution = Vector2::new(config.width, config.height);
    let mut film = Film::new(resolution);
    let camera = load_camera(&config.camera, resolution);

    let filter = MitchellFilter::new(2.0, 2.0, 1.0 / 3.0, 1.0 / 3.0);
    let sampler = load_sampler(&config.sampler);

    ray_tracer::render(
        &scene,
        &camera,
        &mut film,
        &filter,
        &sampler,
        &OriginalRayTracer {},
        5,
    );
    let mut image = film.write_image();

    if config.crop {
        image = crop_to_non_transparent(&image)?;
    }

    Ok(image)
}

/// Renders the STL file to the writer in the PNG formata.
pub fn render_to_writer<R: Read + Seek, W: Write + Seek>(
    stl_file: R,
    config: &Config,
    w: &mut W,
) -> Result<(), Error> {
    let image = render_to_image(stl_file, config)?;
    image.write_to(w, ImageOutputFormat::Png)?;
    Ok(())
}

fn load_mesh<R: Read + Seek>(
    mesh: R,
    mesh_arena: &mut Arena<Mesh>,
    handedness: Handedness,
) -> Result<&Mesh, Error> {
    let mut reader = std::io::BufReader::new(mesh);
    let mesh = mesh_arena.alloc(MeshBuilder::from_stl(&mut reader)?.build());
    let (bounds_min, bounds_max) = mesh.bounding_box().ok_or(Error::EmptyMesh)?;
    let center = bounds_min + (bounds_max - bounds_min) / 2.0;
    let center_to_origin = Matrix4::from_translation(Point3::new(0.0f32, 0.0f32, 0.0f32) - center);
    mesh.transform(center_to_origin);

    let bounding_sphere_radius = max_distance_from_origin(mesh);
    mesh.transform(Matrix4::from_scale(1.0 / bounding_sphere_radius));

    if handedness == config::Handedness::RightHanded {
        mesh.transform_swapping_handedness(Matrix4::from_nonuniform_scale(1.0, -1.0, 1.0));
    }

    Ok(mesh)
}

fn load_material(material_config: &config::Material) -> Material {
    Material::new(
        RgbaSpectrum::from_rgb(
            material_config.color.r,
            material_config.color.g,
            material_config.color.b,
        ),
        material_config.ambient,
        material_config.diffuse,
        material_config.specular,
        material_config.shininess,
        0.0,
    )
}

fn load_light(light_config: &config::Light) -> Light {
    match light_config {
        config::Light::PointLight {
            position,
            intensity,
        } => {
            let light_position = origin_to_spherical_position(
                position.radius,
                Deg(position.theta),
                Deg(position.phi),
            )
            .transform_point(point3(0.0, 0.0, 0.0));
            Light::point_light(
                light_position,
                RgbaSpectrum::from_rgb(intensity.r, intensity.g, intensity.b),
            )
        }
    }
}

fn load_camera(camera_config: &config::Camera, resolution: Vector2<usize>) -> OrthographicCamera {
    // TODO: Return Camera trait object instead.
    match camera_config {
        config::Camera::OrthographicCamera {
            position,
            z_near,
            z_far,
        } => {
            let camera_to_world = origin_to_spherical_position(
                position.radius,
                Deg(position.theta),
                Deg(position.phi),
            );
            OrthographicCamera::new(
                camera_to_world,
                *z_near,
                *z_far,
                orthographic_screen_size(resolution.x as f32 / resolution.y as f32),
                resolution,
            )
        }
    }
}

fn load_sampler(sampler_config: &config::Sampler) -> StratifiedSampler {
    match sampler_config {
        config::Sampler::StratifiedSampler {
            x_strata_count,
            y_strata_count,
            jitter,
        } => StratifiedSampler::new(*x_strata_count, *y_strata_count, 5, 0, *jitter),
    }
}

/// Return the screen size necessary for an orthographic camera with the given
/// aspect ratio to fit a unit sphere centered at the origin.
fn orthographic_screen_size(aspect_ratio: f32) -> Vector2<f32> {
    let diameter = 2.0;
    if aspect_ratio >= 1.0 {
        vec2(aspect_ratio * diameter, diameter)
    } else {
        vec2(diameter, 1.0 / aspect_ratio * diameter)
    }
}

/// Returns a transformation matrix that translates a point at the origin to the
/// given spherical coordinates.
///
/// If this transformation matrix is applied to a camera coordinate system that
/// looks towards positive z and has positive y as its "up" direction, then the
/// resulting coordinate system will look at the origin and "up" will be in
/// roughly the positive y direction.
fn origin_to_spherical_position(r: f32, theta: Deg<f32>, phi: Deg<f32>) -> Matrix4<f32> {
    Matrix4::from_angle_z(Rad(0.0) - Rad(FRAC_PI_2) - Rad::from(phi))
        * Matrix4::from_angle_x(Rad(PI) - Rad::from(theta))
        * Matrix4::from_translation(Vector3::new(0.0, 0.0, -1.0 * r))
}

/// Return the maximum distance between any vertex and the origin.
fn max_distance_from_origin(mesh: &Mesh) -> f32 {
    mesh.positions
        .iter()
        .fold(0.0f32, |acc, p| {
            let dist2 = (p - point3(0.0, 0.0, 0.0)).magnitude2();
            acc.max(dist2)
        })
        .sqrt()
}

/// Crop transparent edges from the image.
fn crop_to_non_transparent(
    image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, Error> {
    let (crop_bounds_min, crop_bounds_max) =
        non_transparent_bounds(image).ok_or(Error::ZeroAreaImage)?;
    let crop_bounds_diag = crop_bounds_max - crop_bounds_min;
    let cropped = imageops::crop_imm(
        image,
        crop_bounds_min.x,
        crop_bounds_min.y,
        crop_bounds_diag.x + 1,
        crop_bounds_diag.y + 1,
    )
    .to_image();
    Ok(cropped)
}

/// Return the min and max (inclusive) pixels of a 2D bounding box around any
/// non-transparent content in the image.
fn non_transparent_bounds(
    image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
) -> Option<(Point2<u32>, Point2<u32>)> {
    let mut min_max = None;

    for (x, y, rgba) in image.enumerate_pixels() {
        if rgba.0[3] == 0 {
            continue;
        }

        min_max = match min_max {
            None => Some((point2(x, y), point2(x, y))),
            Some((min, max)) => {
                let new_min = point2(cmp::min(min.x, x), cmp::min(min.y, y));
                let new_max = point2(cmp::max(max.x, x), cmp::max(max.y, y));
                Some((new_min, new_max))
            }
        };
    }

    min_max
}
