use super::Fetcher;
use anyhow::Result;
use async_trait::async_trait;

pub struct NetFetcher<'a>(pub &'a str);

#[async_trait]
impl<'a> Fetcher for NetFetcher<'a> {
    async fn fetch(&self) -> Result<String> {
        Ok(reqwest::get(self.0).await?.text().await?)
    }
}
