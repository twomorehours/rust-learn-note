use std::ops::{Deref, DerefMut};

use polars::prelude::*;

#[derive(Debug)]
pub struct Dataset(pub DataFrame);

impl Deref for Dataset {
    type Target = DataFrame;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Dataset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
