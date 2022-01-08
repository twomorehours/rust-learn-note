mod routeguide;
use std::hash::Hash;

pub use routeguide::*;

impl Feature {
    pub fn new(location: Option<Point>, name: impl Into<String>) -> Self {
        Self {
            location,
            name: name.into(),
        }
    }
}

impl Point {
    pub fn new(latitude: i32, longitude: i32) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

impl Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.latitude.hash(state);
        self.longitude.hash(state);
    }
}

impl Eq for Point {}
