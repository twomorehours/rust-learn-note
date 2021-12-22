use image::ImageOutputFormat;

use crate::pb::Spec;

mod photon;
pub use photon::*;

pub trait Engine {
    fn apply(&mut self, specs: &[Spec]);
    fn generate(self, format: ImageOutputFormat) -> Vec<u8>;
}
