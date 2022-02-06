mod items;
pub use items::*;

impl Shirt {
    pub fn new(color: impl Into<String>, size: shirt::Size) -> Self {
        Self {
            color: color.into(),
            size: size.into(),
        }
    }
}
