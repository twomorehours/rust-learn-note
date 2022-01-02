use anyhow::Result;
use futures::{SinkExt, StreamExt};
use kv::{CommandRequest, CommandResponse};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let stream = TcpStream::connect("localhost:9527").await?;

    let mut stream = Framed::new(stream, LengthDelimitedCodec::new());

    let req = CommandRequest::new_hget("table1", "hello");

    stream.send(req.clone().into()).await?;
    if let Some(Ok(data)) = stream.next().await {
        let resp = CommandResponse::try_from(data)?;
        info!("recv resp {:?}", resp);
    }

    stream
        .send(CommandRequest::new_hset("table1", "hello", "world".into()).into())
        .await?;
    if let Some(Ok(data)) = stream.next().await {
        let resp = CommandResponse::try_from(data)?;
        info!("recv resp {:?}", resp);
    }

    stream
        .send(CommandRequest::new_hget("table1", "hello").into())
        .await?;
    if let Some(Ok(data)) = stream.next().await {
        let resp = CommandResponse::try_from(data)?;
        info!("recv resp {:?}", resp);
    }

    stream.send(req.into()).await?;
    if let Some(Ok(data)) = stream.next().await {
        let resp = CommandResponse::try_from(data)?;
        info!("recv resp {:?}", resp);
    }

    Ok(())
}
