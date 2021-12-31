use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::{SinkExt, StreamExt};
use kv::{CommandRequest, CommandResponse};
use tokio::net::TcpStream;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let stream = TcpStream::connect("localhost:9527").await?;

    let mut stream: AsyncProstStream<_, CommandResponse, CommandRequest, _> =
        AsyncProstStream::from(stream).for_async();

    stream
        .send(CommandRequest::new_hget("table1", "hello"))
        .await?;
    if let Some(Ok(resp)) = stream.next().await {
        info!("recv resp {:?}", resp);
    }

    stream
        .send(CommandRequest::new_hset("table1", "hello", "world".into()))
        .await?;
    if let Some(Ok(resp)) = stream.next().await {
        info!("recv resp {:?}", resp);
    }

    stream
        .send(CommandRequest::new_hget("table1", "hello"))
        .await?;
    if let Some(Ok(resp)) = stream.next().await {
        info!("recv resp {:?}", resp);
    }

    stream.send(CommandRequest::new_hgetall("table1")).await?;
    if let Some(Ok(resp)) = stream.next().await {
        info!("recv resp {:?}", resp);
    }

    Ok(())
}
