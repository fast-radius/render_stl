#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io {
        #[from]
        source: std::io::Error,
    },

    #[error(transparent)]
    Mesh {
        #[from]
        source: mesh::Error,
    },

    #[error(transparent)]
    ParseInt {
        #[from]
        source: std::num::ParseIntError,
    },

    #[error(transparent)]
    ParseFloat {
        #[from]
        source: std::num::ParseFloatError,
    },

    #[error(transparent)]
    Image {
        #[from]
        source: image::ImageError,
    },

    #[error("cannot render mesh because it is empty")]
    EmptyMesh,

    #[error("image has an area of zero after cropping transparent pixels")]
    ZeroAreaImage,
}
