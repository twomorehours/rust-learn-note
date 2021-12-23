use crate::dataset::Dataset;
use anyhow::Result;

mod csv;
use csv::*;

pub trait Loader {
    fn load(self) -> Result<Dataset>;
}

pub enum DataLoader {
    CsvLoader(CsvLoader),
}

pub fn detect_content(content: String) -> DataLoader {
    DataLoader::CsvLoader(CsvLoader(content))
}

impl Loader for DataLoader {
    fn load(self) -> Result<Dataset> {
        match self {
            DataLoader::CsvLoader(csv) => csv.load(),
        }
    }
}
