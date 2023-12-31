mod triangle;

use cgmath::{num_traits::identities, InnerSpace, Matrix4, Point2, Point3, Transform, Vector3};
pub use triangle::Triangle;

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct Error {
    #[from]
    source: nom_stl::Error,
}

/// A mesh of triangles.
#[derive(Debug)]
pub struct Mesh {
    /// Contains a position for each vertex in the mesh.
    pub positions: Vec<Point3<f32>>,

    /// Contains a normal vector for each vertex in the mesh.
    pub normals: Vec<Vector3<f32>>,

    /// Contains a UV coordinate for each vertex in the mesh.
    pub uvs: Option<Vec<Point2<f32>>>,

    /// An array that describes each triangle in the mesh. Each element of the
    /// array is a tuple that contains three indices into the `vertices` array.
    pub triangle_vertex_indices: Vec<(usize, usize, usize)>,

    pub transformation_swaps_handedness: bool,
    pub reverse_orientation: bool,
}

impl Mesh {
    /// Apply the transformation matrix to the position and normal of each
    /// vertex in the mesh.
    pub fn transform(&mut self, transformation: Matrix4<f32>) {
        for p in &mut self.positions {
            *p = transformation.transform_point(*p);
        }

        for n in &mut self.normals {
            // TODO: Account for reversing orientation and swapping handedness.
            *n = transformation.transform_vector(*n).normalize();
        }
    }

    /// Apply the transformation matrix to the position and normal of each
    /// vertex in the mesh. Flip the value of `transformation_swaps_handedness`.
    pub fn transform_swapping_handedness(&mut self, transformation: Matrix4<f32>) {
        self.transform(transformation);
        self.transformation_swaps_handedness = !self.transformation_swaps_handedness
    }

    /// Returns the minimum and maximum corners of an axis-aligned bounded box
    /// around the mesh.
    pub fn bounding_box(&self) -> Option<(Point3<f32>, Point3<f32>)> {
        if self.positions.is_empty() {
            return None;
        }

        let mut min = self.positions[0];
        let mut max = self.positions[0];

        for p in &self.positions {
            if p.x < min.x {
                min.x = p.x;
            }
            if p.y < min.y {
                min.y = p.y;
            }
            if p.z < min.z {
                min.z = p.z;
            }
            if p.x > max.x {
                max.x = p.x;
            }
            if p.y > max.y {
                max.y = p.y;
            }
            if p.z > max.z {
                max.z = p.z;
            }
        }

        Some((min, max))
    }
}

pub struct MeshBuilder {
    positions: Vec<Point3<f32>>,
    normals: Vec<Vector3<f32>>,
    uvs: Option<Vec<Point2<f32>>>,
    triangle_vertex_indices: Vec<(usize, usize, usize)>,

    transformation: Matrix4<f32>,
    transformation_swaps_handedness: bool,
    reverse_orientation: bool,
}

impl MeshBuilder {
    pub fn new(
        positions: Vec<Point3<f32>>,
        normals: Vec<Vector3<f32>>,
        triangle_vertex_indices: Vec<(usize, usize, usize)>,
    ) -> Self {
        Self {
            positions,
            normals,
            uvs: None,
            triangle_vertex_indices,
            transformation: identities::one(),
            transformation_swaps_handedness: false,
            reverse_orientation: false,
        }
    }

    pub fn uvs(mut self, uvs: Vec<Point2<f32>>) -> Self {
        self.uvs = Some(uvs);
        self
    }

    pub fn transformation(mut self, transformation: Matrix4<f32>) -> Self {
        self.transformation = transformation;
        self
    }

    pub fn transformation_swaps_handedness(mut self, transform_swaps_handedness: bool) -> Self {
        self.transformation_swaps_handedness = transform_swaps_handedness;
        self
    }

    pub fn reverse_orientation(mut self, reverse_orientation: bool) -> Self {
        self.reverse_orientation = reverse_orientation;
        self
    }

    pub fn build(self) -> Mesh {
        let mut mesh = Mesh {
            positions: self.positions,
            normals: self.normals,
            uvs: self.uvs,
            triangle_vertex_indices: self.triangle_vertex_indices,
            transformation_swaps_handedness: self.transformation_swaps_handedness,
            reverse_orientation: self.reverse_orientation,
        };
        mesh.transform(self.transformation);
        mesh
    }

    pub fn from_stl<R>(stl_bytes: &mut R) -> Result<MeshBuilder, Error>
    where
        R: std::io::Read + std::io::Seek,
    {
        let stl = nom_stl::parse_stl(stl_bytes)?;
        let num_triangles = stl.triangles().len();

        let mut positions = vec![Point3::new(0.0, 0.0, 0.0); num_triangles * 3];
        let mut normals = vec![Vector3::new(0.0, 0.0, 0.0); num_triangles * 3];
        let mut triangle_vertex_indices = vec![(0, 0, 0); num_triangles];

        for (i, t) in stl.triangles().iter().enumerate() {
            let [[v1x, v1y, v1z], [v2x, v2y, v2z], [v3x, v3y, v3z]] = t.vertices();
            positions[3 * i] = Point3::new(v1x, v1y, v1z);
            positions[(3 * i) + 1] = Point3::new(v2x, v2y, v2z);
            positions[(3 * i) + 2] = Point3::new(v3x, v3y, v3z);

            let [nx, ny, nz] = t.normal();
            let normal = Vector3::new(nx, ny, nz);
            normals[3 * i] = normal;
            normals[(3 * i) + 1] = normal;
            normals[(3 * i) + 2] = normal;

            triangle_vertex_indices[i] = (3 * i, (3 * i) + 1, (3 * i) + 2);
        }

        Ok(MeshBuilder::new(
            positions,
            normals,
            triangle_vertex_indices,
        ))
    }
}
