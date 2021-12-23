use crate::dataset::Dataset;
use polars::prelude::*;
use std::io::Cursor;

use super::Loader;

pub struct CsvLoader(pub String);

impl Loader for CsvLoader {
    fn load(self) -> anyhow::Result<Dataset> {
        let df = CsvReader::new(Cursor::new(self.0))
            .infer_schema(Some(16))
            .finish()?;
        Ok(Dataset(df))
    }
}
