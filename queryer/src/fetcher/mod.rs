use anyhow::Result;
use async_trait::async_trait;

mod file;
use file::*;

mod net;
use net::*;

#[async_trait]
pub trait Fetcher {
    async fn fetch(&self) -> Result<String>;
}

pub async fn retrieve_data(source: &str) -> Result<String> {
    match &source[..4] {
        "http" => NetFetcher(source).fetch().await,
        _ => FileFetcher(source).fetch().await,
    }
}
