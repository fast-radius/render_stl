pub mod camera;
pub mod color;
pub mod film;
pub mod filter;
mod geometry;
pub mod integrator;
mod interaction;
pub mod light;
mod number;
mod ray;
pub mod sampler;
pub mod simple;
mod triangle;

pub use integrator::render;

#[cfg(test)]
mod test;
