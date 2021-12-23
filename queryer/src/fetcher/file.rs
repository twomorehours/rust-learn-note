// 接口的使用
// 1. mod提供一个函数 mod内部使用不同的实现就完成
// 2. mod提供一个enum， enum里面的子类型是各个实现， enum本身也实现了接口。 函数返回enum。
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
