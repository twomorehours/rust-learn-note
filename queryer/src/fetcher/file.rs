use super::Fetcher;
use anyhow::Result;
use async_trait::async_trait;
use tokio::fs;

pub struct FileFetcher<'a>(pub &'a str);

#[async_trait]
impl<'a> Fetcher for FileFetcher<'a> {
    async fn fetch(&self) -> Result<String> {
        Ok(fs::read_to_string(self.0).await?)
    }
}
